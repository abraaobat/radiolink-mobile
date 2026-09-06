#!/usr/bin/env bash
set -Eeuo pipefail

VERSION="1.1.0"

usage() {
  cat <<'EOF'
RadioLink YouTube Corpus Builder

Cria um corpus local de pesquisa a partir de um canal do YouTube:
- enumera videos/shorts/streams;
- tenta obter legendas manuais/automaticas primeiro;
- se nao houver legenda, baixa apenas o audio e transcreve localmente com whisper.cpp;
- preserva metadata, VTT bruto e uma transcricao temporizada uniforme;
- gera transcript.txt, transcript.vtt, transcript.segments.jsonl, transcript.md, index.csv, index.jsonl e corpus.jsonl;
- e incremental: videos ja processados sao ignorados, salvo com --force.

USO
  ./radiolink-youtube-corpus.sh --setup
  ./radiolink-youtube-corpus.sh CHANNEL_URL [opcoes]

EXEMPLO (The Tech Prepper)
  ./radiolink-youtube-corpus.sh \
    "https://www.youtube.com/@TheTechPrepper" \
    --lang en \
    --output "$HOME/RadioLinkResearch/youtube/the-tech-prepper"

OPCOES
  --output DIR                 Diretorio do corpus.
  --lang LANG                  Idioma falado/transcrito. Padrao: en
  --tabs LIST                  Abas separadas por virgula. Padrao: videos,shorts,streams
  --model NAME                 Modelo whisper.cpp. Padrao: large-v3-turbo-q5_0
  --model-path FILE            Caminho de um modelo GGML ja existente.
  --cookies-from-browser NAME  Ex.: chrome, firefox, safari (quando o YouTube exigir login).
  --limit N                    Processa somente os N primeiros itens (bom para teste).
  --no-whisper                 Nao usa fallback local; salva apenas legendas existentes.
  --keep-audio                 Mantem audio WAV usado pelo Whisper em videos/<id>/audio.wav.
  --force                      Reprocessa mesmo que transcript.txt ja exista.
  --setup                      Instala yt-dlp, ffmpeg e whisper-cpp via Homebrew.
  -h, --help                   Ajuda.

VARIAVEIS DE AMBIENTE
  RADIOLINK_WHISPER_MODEL_PATH Caminho alternativo do modelo GGML.

NOTA
  Mantenha o corpus bruto fora do Git quando ele contiver transcricoes integrais de terceiros.
  Para o repositorio RadioLink, prefira versionar indices, URLs, notas e sinteses derivadas.
EOF
}

log() { printf '[RadioLink] %s\n' "$*"; }
warn() { printf '[RadioLink][aviso] %s\n' "$*" >&2; }
die() { printf '[RadioLink][erro] %s\n' "$*" >&2; exit 1; }

setup_deps() {
  command -v brew >/dev/null 2>&1 || die "Homebrew nao encontrado. Instale em https://brew.sh e rode novamente."
  log "Instalando/atualizando dependencias..."
  brew install yt-dlp ffmpeg whisper-cpp
  log "Dependencias prontas."
  log "Agora execute o script com a URL do canal."
}

if [[ "${1:-}" == "--setup" ]]; then
  setup_deps
  exit 0
fi

CHANNEL_URL=""
OUT_DIR=""
LANG_CODE="en"
TABS="videos,shorts,streams"
MODEL_NAME="large-v3-turbo-q5_0"
MODEL_PATH="${RADIOLINK_WHISPER_MODEL_PATH:-}"
COOKIES_BROWSER=""
LIMIT=0
USE_WHISPER=1
KEEP_AUDIO=0
FORCE=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    --output)
      [[ $# -ge 2 ]] || die "--output exige um diretorio"
      OUT_DIR="$2"; shift 2 ;;
    --lang)
      [[ $# -ge 2 ]] || die "--lang exige um codigo, ex.: en ou pt"
      LANG_CODE="$2"; shift 2 ;;
    --tabs)
      [[ $# -ge 2 ]] || die "--tabs exige uma lista, ex.: videos,shorts,streams"
      TABS="$2"; shift 2 ;;
    --model)
      [[ $# -ge 2 ]] || die "--model exige um nome"
      MODEL_NAME="$2"; shift 2 ;;
    --model-path)
      [[ $# -ge 2 ]] || die "--model-path exige um arquivo"
      MODEL_PATH="$2"; shift 2 ;;
    --cookies-from-browser)
      [[ $# -ge 2 ]] || die "--cookies-from-browser exige um navegador"
      COOKIES_BROWSER="$2"; shift 2 ;;
    --limit)
      [[ $# -ge 2 ]] || die "--limit exige um numero"
      LIMIT="$2"; shift 2 ;;
    --no-whisper)
      USE_WHISPER=0; shift ;;
    --keep-audio)
      KEEP_AUDIO=1; shift ;;
    --force)
      FORCE=1; shift ;;
    -h|--help)
      usage; exit 0 ;;
    --*)
      die "Opcao desconhecida: $1" ;;
    *)
      if [[ -z "$CHANNEL_URL" ]]; then
        CHANNEL_URL="$1"; shift
      else
        die "Argumento inesperado: $1"
      fi ;;
  esac
done

[[ -n "$CHANNEL_URL" ]] || { usage; exit 1; }
[[ "$LIMIT" =~ ^[0-9]+$ ]] || die "--limit deve ser um inteiro >= 0"

for cmd in yt-dlp ffmpeg ffprobe python3 curl; do
  command -v "$cmd" >/dev/null 2>&1 || die "Dependencia ausente: $cmd. Rode: $0 --setup"
done

if (( USE_WHISPER )); then
  command -v whisper-cli >/dev/null 2>&1 || die "whisper-cli ausente. Rode: $0 --setup"
fi

# Canonicaliza a URL removendo uma aba final, se houver.
BASE_URL="${CHANNEL_URL%/}"
for suffix in videos shorts streams; do
  if [[ "$BASE_URL" == */"$suffix" ]]; then
    BASE_URL="${BASE_URL%/$suffix}"
  fi
done

slug_from_url() {
  local u="$1" s
  s="${u##*/}"
  s="${s#@}"
  [[ -n "$s" ]] || s="youtube-channel"
  printf '%s' "$s" | tr '[:upper:]' '[:lower:]' | tr -cs 'a-z0-9._-' '-'
}

CHANNEL_SLUG="$(slug_from_url "$BASE_URL")"
if [[ -z "$OUT_DIR" ]]; then
  OUT_DIR="$PWD/radiolink-research/youtube/$CHANNEL_SLUG"
fi

VIDEOS_DIR="$OUT_DIR/videos"
STATE_DIR="$OUT_DIR/state"
LOG_DIR="$OUT_DIR/logs"
CORPUS_DIR="$OUT_DIR/corpus"
mkdir -p "$VIDEOS_DIR" "$STATE_DIR" "$LOG_DIR" "$CORPUS_DIR"

RUN_TS="$(date '+%Y%m%d-%H%M%S')"
RUN_LOG="$LOG_DIR/run-$RUN_TS.log"
exec > >(tee -a "$RUN_LOG") 2>&1

log "RadioLink YouTube Corpus Builder v$VERSION"
log "Canal: $BASE_URL"
log "Saida: $OUT_DIR"
log "Idioma: $LANG_CODE"
log "Abas: $TABS"

# Wrapper compatível com o Bash 3.2 que acompanha o macOS.
# Com `set -u`, expandir um array vazio (ex.: "${ARRAY[@]}") pode causar
# "unbound variable" no Bash antigo. Por isso a opção de cookies é
# adicionada condicionalmente, sem arrays opcionais.
ytdlp() {
  if [[ -n "$COOKIES_BROWSER" ]]; then
    command yt-dlp --cookies-from-browser "$COOKIES_BROWSER" "$@"
  else
    command yt-dlp "$@"
  fi
}

if [[ -n "$COOKIES_BROWSER" ]]; then
  log "Cookies: $COOKIES_BROWSER"
fi

ensure_model() {
  if [[ -n "$MODEL_PATH" ]]; then
    [[ -f "$MODEL_PATH" ]] || die "Modelo nao encontrado em: $MODEL_PATH"
    return
  fi

  MODEL_DIR="$HOME/Library/Caches/RadioLink/whisper"
  MODEL_PATH="$MODEL_DIR/ggml-$MODEL_NAME.bin"
  mkdir -p "$MODEL_DIR"

  if [[ -s "$MODEL_PATH" ]]; then
    log "Modelo Whisper: $MODEL_PATH"
    return
  fi

  local part="$MODEL_PATH.part"
  local url="https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-$MODEL_NAME.bin"
  log "Baixando modelo Whisper '$MODEL_NAME' (somente na primeira execucao)..."
  # -C - permite retomar download parcial quando o servidor suporta Range.
  if ! curl -fL --retry 4 --retry-delay 2 -C - -o "$part" "$url"; then
    rm -f "$part"
    die "Falha ao baixar o modelo Whisper: $url"
  fi
  mv "$part" "$MODEL_PATH"
  [[ -s "$MODEL_PATH" ]] || die "Modelo Whisper baixado, mas o arquivo esta vazio"
  log "Modelo salvo em: $MODEL_PATH"
}

if (( USE_WHISPER )); then
  ensure_model
fi

# Lista de IDs, preservando a primeira ocorrencia entre as abas.
RAW_IDS="$STATE_DIR/video_ids.raw.txt"
ALL_IDS="$STATE_DIR/video_ids.txt"
: > "$RAW_IDS"

IFS=',' read -r -a TAB_ARRAY <<< "$TABS"
for tab in "${TAB_ARRAY[@]}"; do
  tab="${tab//[[:space:]]/}"
  [[ -n "$tab" ]] || continue
  TAB_URL="$BASE_URL/$tab"
  log "Enumerando: $TAB_URL"
  ytdlp \
    --flat-playlist \
    --ignore-errors \
    --no-warnings \
    --print '%(id)s' \
    "$TAB_URL" >> "$RAW_IDS" || warn "Nao foi possivel enumerar completamente a aba '$tab'"
done

awk 'NF && !seen[$0]++' "$RAW_IDS" > "$ALL_IDS"
rm -f "$RAW_IDS"

TOTAL_ALL="$(wc -l < "$ALL_IDS" | tr -d ' ')"
[[ "$TOTAL_ALL" -gt 0 ]] || die "Nenhum video encontrado. Tente --cookies-from-browser chrome (ou firefox/safari)."

WORK_IDS="$ALL_IDS"
if (( LIMIT > 0 && LIMIT < TOTAL_ALL )); then
  WORK_IDS="$STATE_DIR/video_ids.limit.txt"
  sed -n "1,${LIMIT}p" "$ALL_IDS" > "$WORK_IDS"
fi
TOTAL="$(wc -l < "$WORK_IDS" | tr -d ' ')"
log "Encontrados: $TOTAL_ALL itens; processando nesta execucao: $TOTAL"

clean_vtt() {
  local input="$1" output="$2"
  python3 - "$input" "$output" <<'PY'
from pathlib import Path
import html
import re
import sys

src = Path(sys.argv[1])
dst = Path(sys.argv[2])
raw = src.read_text(encoding="utf-8", errors="replace").replace("\r\n", "\n")

# Remove cabecalhos STYLE/REGION/NOTE de forma simples e extrai texto dos cues.
blocks = re.split(r"\n\s*\n", raw)
cues = []
for block in blocks:
    lines = [ln.strip() for ln in block.splitlines()]
    if not lines:
        continue
    if lines[0].startswith(("WEBVTT", "STYLE", "REGION", "NOTE")):
        continue

    ts_idx = None
    for i, ln in enumerate(lines):
        if "-->" in ln:
            ts_idx = i
            break
    if ts_idx is None:
        continue

    text = " ".join(lines[ts_idx + 1:]).strip()
    if not text:
        continue

    # Remove timestamps inline do YouTube, tags VTT/HTML e entidades.
    text = re.sub(r"<\d{2}:\d{2}:\d{2}\.\d{3}>", " ", text)
    text = re.sub(r"<[^>]+>", " ", text)
    text = html.unescape(text)
    text = re.sub(r"\s+", " ", text).strip()

    if not text:
        continue
    if re.fullmatch(r"[\[(]?(music|applause|laughter|silence)[\])]?[.!]?", text, re.I):
        continue
    cues.append(text)

# Deduplicacao para captions automaticas do YouTube, que frequentemente usam janelas sobrepostas.
def norm_token(t: str) -> str:
    return re.sub(r"(^\W+|\W+$)", "", t).lower()

out_tokens = []
for cue in cues:
    toks = cue.split()
    if not toks:
        continue

    # Se o cue inteiro ja estiver contido no final recente, ignora.
    norm_new = [norm_token(t) for t in toks]
    recent = [norm_token(t) for t in out_tokens[-max(80, len(toks) * 2):]]
    joined_new = " ".join(norm_new)
    joined_recent = " ".join(recent)
    if joined_new and joined_new in joined_recent:
        continue

    max_k = min(40, len(out_tokens), len(toks))
    overlap = 0
    for k in range(max_k, 0, -1):
        a = [norm_token(t) for t in out_tokens[-k:]]
        b = [norm_token(t) for t in toks[:k]]
        if a == b:
            overlap = k
            break
    out_tokens.extend(toks[overlap:])

text = " ".join(out_tokens)
text = re.sub(r"\s+([,.;:!?])", r"\1", text)
text = re.sub(r"\s+", " ", text).strip()

# Quebra em linhas por frases para facilitar leitura e indexacao.
text = re.sub(r"(?<=[.!?])\s+(?=[A-Z0-9])", "\n", text)
if text:
    text += "\n"
dst.write_text(text, encoding="utf-8")
PY
}

build_timed_segments() {
  local input="$1" output="$2" video_id="$3"
  python3 - "$input" "$output" "$video_id" <<'PY'
from pathlib import Path
import html
import json
import re
import sys

src = Path(sys.argv[1])
dst = Path(sys.argv[2])
video_id = sys.argv[3]
raw = src.read_text(encoding="utf-8", errors="replace").replace("\r\n", "\n")

TS_RE = re.compile(
    r"(?P<start>(?:\d{2}:)?\d{2}:\d{2}\.\d{3})\s*-->\s*"
    r"(?P<end>(?:\d{2}:)?\d{2}:\d{2}\.\d{3})"
)

def seconds(ts: str) -> float:
    parts = ts.split(":")
    if len(parts) == 3:
        h, m, sec = parts
    else:
        h = "0"
        m, sec = parts
    return int(h) * 3600 + int(m) * 60 + float(sec)

def clean_text(lines):
    text = " ".join(lines).strip()
    text = re.sub(r"<\d{2}:\d{2}:\d{2}\.\d{3}>", " ", text)
    text = re.sub(r"<[^>]+>", " ", text)
    text = html.unescape(text)
    return re.sub(r"\s+", " ", text).strip()

rows = []
for block in re.split(r"\n\s*\n", raw):
    lines = [ln.strip() for ln in block.splitlines() if ln.strip()]
    if not lines or lines[0].startswith(("WEBVTT", "STYLE", "REGION", "NOTE")):
        continue
    ts_idx = None
    match = None
    for i, line in enumerate(lines):
        m = TS_RE.search(line)
        if m:
            ts_idx = i
            match = m
            break
    if match is None:
        continue
    text = clean_text(lines[ts_idx + 1:])
    if not text:
        continue
    start = match.group("start")
    end = match.group("end")
    start_s = seconds(start)
    end_s = seconds(end)
    rows.append({
        "segment_index": len(rows),
        "start": start,
        "end": end,
        "start_seconds": round(start_s, 3),
        "end_seconds": round(end_s, 3),
        "text": text,
        "video_id": video_id,
        "url": f"https://www.youtube.com/watch?v={video_id}&t={int(start_s)}s",
    })

with dst.open("w", encoding="utf-8") as f:
    for row in rows:
        f.write(json.dumps(row, ensure_ascii=False) + "\n")
PY
}

write_status() {
  local status_file="$1" source="$2" vtt_file="${3:-}"
  python3 - "$status_file" "$source" "$LANG_CODE" "$MODEL_NAME" "$vtt_file" <<'PY'
import json, sys
from datetime import datetime, timezone
from pathlib import Path

path, source, lang, model, vtt = sys.argv[1:]
data = {
    "transcript_source": source,
    "language": lang,
    "whisper_model": model if source == "whisper.cpp" else None,
    "caption_file": Path(vtt).name if vtt else None,
    "timed_transcript_file": "transcript.vtt" if (Path(path).parent / "transcript.vtt").exists() else None,
    "segments_file": "transcript.segments.jsonl" if (Path(path).parent / "transcript.segments.jsonl").exists() else None,
    "generated_at": datetime.now(timezone.utc).isoformat(),
}
Path(path).write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
PY
}

build_markdown() {
  local video_dir="$1"
  python3 - "$video_dir" <<'PY'
import json
from pathlib import Path
import sys

vd = Path(sys.argv[1])
meta = json.loads((vd / "metadata.json").read_text(encoding="utf-8"))
status = json.loads((vd / "status.json").read_text(encoding="utf-8"))
text = (vd / "transcript.txt").read_text(encoding="utf-8", errors="replace").strip()

def val(*keys, default=""):
    for k in keys:
        v = meta.get(k)
        if v is not None and v != "":
            return v
    return default

vid = val("id")
url = val("webpage_url", "original_url", default=f"https://www.youtube.com/watch?v={vid}")
title = val("title", default=vid)
channel = val("channel", "uploader")
upload_date = val("upload_date")
duration = val("duration")
desc = val("description")

lines = [
    f"# {title}",
    "",
    f"- **Channel:** {channel}",
    f"- **Video ID:** `{vid}`",
    f"- **URL:** {url}",
    f"- **Upload date:** {upload_date}",
    f"- **Duration (s):** {duration}",
    f"- **Transcript source:** {status.get('transcript_source', '')}",
    f"- **Language:** {status.get('language', '')}",
]
if status.get("whisper_model"):
    lines.append(f"- **Whisper model:** {status['whisper_model']}")
if status.get("timed_transcript_file"):
    lines.append(f"- **Timed transcript:** `{status['timed_transcript_file']}`")
if status.get("segments_file"):
    lines.append(f"- **Timed segments:** `{status['segments_file']}`")
if desc:
    lines += ["", "## Video description", "", desc.strip()]
lines += ["", "## Transcript", "", text, ""]
(vd / "transcript.md").write_text("\n".join(lines), encoding="utf-8")
PY
}

process_video() {
  local id="$1" idx="$2"
  local url="https://www.youtube.com/watch?v=$id"
  local vd="$VIDEOS_DIR/$id"
  local tmp="$vd/.tmp"
  mkdir -p "$vd" "$tmp"

  if [[ -s "$vd/transcript.txt" && -s "$vd/status.json" && "$FORCE" -eq 0 ]]; then
    log "[$idx/$TOTAL] $id - ja processado; pulando"
    rm -rf "$tmp"
    return 0
  fi

  log "[$idx/$TOTAL] Processando $url"

  # Metadata independente da etapa de legendas.
  if ! ytdlp \
      --no-playlist \
      --skip-download \
      --ignore-errors \
      --no-warnings \
      --dump-single-json \
      "$url" > "$tmp/metadata.json"; then
    warn "$id - falha ao obter metadata"
  fi

  if [[ ! -s "$tmp/metadata.json" ]]; then
    warn "$id - metadata vazia; video pode estar privado, removido ou exigir login"
    rm -rf "$tmp"
    return 1
  fi
  mv "$tmp/metadata.json" "$vd/metadata.json"

  # Primeiro caminho: legendas oficiais/automaticas do YouTube no idioma solicitado.
  rm -f "$vd"/captions.*.vtt "$vd"/captions.vtt "$vd/transcript.vtt" "$vd/transcript.srt" "$vd/transcript.segments.jsonl" 2>/dev/null || true
  ytdlp \
    --no-playlist \
    --skip-download \
    --ignore-errors \
    --no-warnings \
    --write-subs \
    --write-auto-subs \
    --sub-langs "$LANG_CODE" \
    --sub-format 'vtt/best' \
    -o "$vd/captions.%(ext)s" \
    "$url" >/dev/null 2>&1 || true

  local caption=""
  caption="$(find "$vd" -maxdepth 1 -type f -name 'captions*.vtt' -print | head -n 1 || true)"

  if [[ -n "$caption" && -s "$caption" ]]; then
    clean_vtt "$caption" "$vd/transcript.txt"
    if [[ -s "$vd/transcript.txt" ]]; then
      cp "$caption" "$vd/transcript.vtt"
      build_timed_segments "$vd/transcript.vtt" "$vd/transcript.segments.jsonl" "$id"
      write_status "$vd/status.json" "youtube-captions" "$caption"
      build_markdown "$vd"
      log "$id - transcricao obtida das legendas do YouTube"
      rm -rf "$tmp"
      return 0
    fi
  fi

  if (( ! USE_WHISPER )); then
    warn "$id - sem legenda em '$LANG_CODE' e fallback Whisper desativado"
    rm -rf "$tmp"
    return 1
  fi

  # Segundo caminho: baixa somente o audio, normaliza para WAV PCM 16 kHz mono e roda Whisper local.
  log "$id - sem legenda utilizavel; usando whisper.cpp"
  rm -f "$tmp"/audio.* "$tmp/audio.wav" 2>/dev/null || true
  if ! ytdlp \
      --no-playlist \
      --ignore-errors \
      --no-warnings \
      -f 'bestaudio/best' \
      -o "$tmp/audio.%(ext)s" \
      "$url" >/dev/null; then
    warn "$id - falha no download do audio"
  fi

  local audio=""
  audio="$(find "$tmp" -maxdepth 1 -type f -name 'audio.*' ! -name 'audio.wav' -print | head -n 1 || true)"
  [[ -n "$audio" && -s "$audio" ]] || { warn "$id - arquivo de audio nao encontrado"; rm -rf "$tmp"; return 1; }

  if ! ffmpeg -nostdin -hide_banner -loglevel error -y \
      -i "$audio" -vn -ar 16000 -ac 1 -c:a pcm_s16le "$tmp/audio.wav"; then
    warn "$id - ffmpeg nao conseguiu converter o audio"
    rm -rf "$tmp"
    return 1
  fi

  rm -f "$vd/whisper.txt" "$vd/whisper.vtt" "$vd/whisper.srt"
  whisper-cli \
    -m "$MODEL_PATH" \
    -f "$tmp/audio.wav" \
    -l "$LANG_CODE" \
    -otxt -ovtt -osrt \
    -of "$vd/whisper" >/dev/null 2>&1 || true

  # whisper-cli 1.9.x pode retornar exit code 0 em algumas falhas de decode;
  # por isso verificamos explicitamente a existencia e o tamanho do arquivo.
  if [[ ! -s "$vd/whisper.txt" ]]; then
    warn "$id - Whisper nao gerou uma transcricao valida"
    rm -rf "$tmp"
    return 1
  fi

  mv "$vd/whisper.txt" "$vd/transcript.txt"
  if [[ -s "$vd/whisper.vtt" ]]; then
    mv "$vd/whisper.vtt" "$vd/transcript.vtt"
    build_timed_segments "$vd/transcript.vtt" "$vd/transcript.segments.jsonl" "$id"
  fi
  if [[ -s "$vd/whisper.srt" ]]; then
    mv "$vd/whisper.srt" "$vd/transcript.srt"
  fi
  write_status "$vd/status.json" "whisper.cpp"
  build_markdown "$vd"

  if (( KEEP_AUDIO )); then
    mv "$tmp/audio.wav" "$vd/audio.wav"
  fi
  rm -rf "$tmp"
  log "$id - transcrito localmente com whisper.cpp"
  return 0
}

SUCCESS=0
FAILED=0
IDX=0
while IFS= read -r id; do
  [[ -n "$id" ]] || continue
  IDX=$((IDX + 1))
  if process_video "$id" "$IDX"; then
    SUCCESS=$((SUCCESS + 1))
  else
    FAILED=$((FAILED + 1))
  fi
done < "$WORK_IDS"

# Reconstroi indices e corpus a partir do estado atual, sem duplicatas.
python3 - "$OUT_DIR" "$BASE_URL" "$LANG_CODE" "$MODEL_NAME" "$TABS" <<'PY'
from pathlib import Path
from datetime import datetime, timezone
import csv
import json
import sys

root = Path(sys.argv[1])
base_url, lang, model, tabs = sys.argv[2:]
video_dirs = sorted((root / "videos").glob("*"))
rows = []
corpus_rows = []

for vd in video_dirs:
    mp = vd / "metadata.json"
    sp = vd / "status.json"
    tp = vd / "transcript.txt"
    mdp = vd / "transcript.md"
    if not (mp.exists() and sp.exists() and tp.exists() and tp.stat().st_size):
        continue
    try:
        meta = json.loads(mp.read_text(encoding="utf-8"))
        status = json.loads(sp.read_text(encoding="utf-8"))
    except Exception:
        continue

    vid = str(meta.get("id") or vd.name)
    url = meta.get("webpage_url") or meta.get("original_url") or f"https://www.youtube.com/watch?v={vid}"
    row = {
        "video_id": vid,
        "title": meta.get("title") or "",
        "channel": meta.get("channel") or meta.get("uploader") or "",
        "upload_date": meta.get("upload_date") or "",
        "duration_seconds": meta.get("duration") or "",
        "url": url,
        "transcript_source": status.get("transcript_source") or "",
        "language": status.get("language") or "",
        "transcript_path": str(tp.relative_to(root)),
        "timed_transcript_path": str((vd / "transcript.vtt").relative_to(root)) if (vd / "transcript.vtt").exists() else "",
        "segments_path": str((vd / "transcript.segments.jsonl").relative_to(root)) if (vd / "transcript.segments.jsonl").exists() else "",
        "markdown_path": str(mdp.relative_to(root)) if mdp.exists() else "",
    }
    rows.append(row)
    corpus_rows.append({**row, "description": meta.get("description") or "", "transcript": tp.read_text(encoding="utf-8", errors="replace").strip()})

# Mais recentes primeiro quando upload_date existe; IDs como desempate.
rows.sort(key=lambda r: (r.get("upload_date") or "", r.get("video_id") or ""), reverse=True)
corpus_rows.sort(key=lambda r: (r.get("upload_date") or "", r.get("video_id") or ""), reverse=True)

fieldnames = [
    "video_id", "title", "channel", "upload_date", "duration_seconds", "url",
    "transcript_source", "language", "transcript_path", "timed_transcript_path", "segments_path", "markdown_path"
]
with (root / "index.csv").open("w", encoding="utf-8", newline="") as f:
    w = csv.DictWriter(f, fieldnames=fieldnames)
    w.writeheader()
    w.writerows(rows)

with (root / "index.jsonl").open("w", encoding="utf-8") as f:
    for row in rows:
        f.write(json.dumps(row, ensure_ascii=False) + "\n")

with (root / "corpus" / "corpus.jsonl").open("w", encoding="utf-8") as f:
    for row in corpus_rows:
        f.write(json.dumps(row, ensure_ascii=False) + "\n")

with (root / "corpus" / "all-transcripts.md").open("w", encoding="utf-8") as f:
    f.write("# RadioLink YouTube Research Corpus\n\n")
    for row in corpus_rows:
        f.write(f"## {row['title']}\n\n")
        f.write(f"- Video ID: `{row['video_id']}`\n")
        f.write(f"- URL: {row['url']}\n")
        f.write(f"- Upload date: {row['upload_date']}\n")
        f.write(f"- Transcript source: {row['transcript_source']}\n\n")
        f.write(row["transcript"])
        f.write("\n\n---\n\n")

manifest = {
    "schema_version": 1,
    "generated_at": datetime.now(timezone.utc).isoformat(),
    "channel_url": base_url,
    "language": lang,
    "tabs": tabs.split(","),
    "whisper_model": model,
    "transcribed_items": len(rows),
}
(root / "manifest.json").write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")

readme = f"""# RadioLink YouTube Research Corpus

Corpus local gerado por `radiolink-youtube-corpus.sh`.

- Canal: {base_url}
- Idioma fonte: {lang}
- Itens com transcricao: {len(rows)}
- Modelo de fallback: {model}

## Estrutura

- `videos/<video_id>/metadata.json`: metadata original obtida via yt-dlp.
- `videos/<video_id>/captions.<lang>.vtt`: legenda bruta quando fornecida pelo YouTube.
- `videos/<video_id>/transcript.txt`: texto limpo usado para busca/indexacao.
- `videos/<video_id>/transcript.vtt`: transcricao temporizada uniforme (YouTube ou Whisper).
- `videos/<video_id>/transcript.srt`: SRT quando a fonte for Whisper.
- `videos/<video_id>/transcript.segments.jsonl`: segmentos com inicio/fim e URL direta para o timestamp.
- `videos/<video_id>/transcript.md`: fonte legivel com metadata e URL.
- `index.csv`: indice tabular.
- `index.jsonl`: indice estruturado.
- `corpus/corpus.jsonl`: um registro por video incluindo a transcricao integral.
- `corpus/all-transcripts.md`: corpus consolidado para leitura/busca local.
- `logs/`: logs de execucao.

## Politica sugerida para o RadioLink

Mantenha as transcricoes integrais de terceiros como dados locais de pesquisa e fora do Git.
Versione no repositorio apenas URLs, referencias, notas tecnicas e sinteses derivadas quando apropriado.
"""
(root / "README.md").write_text(readme, encoding="utf-8")
PY

log "Concluido. Sucessos nesta execucao: $SUCCESS | Falhas: $FAILED"
log "Indice: $OUT_DIR/index.csv"
log "Corpus JSONL: $OUT_DIR/corpus/corpus.jsonl"
log "Corpus Markdown: $OUT_DIR/corpus/all-transcripts.md"
log "Log: $RUN_LOG"

if (( FAILED > 0 )); then
  warn "Alguns itens falharam. Rode novamente; o processo e incremental. Para restricoes do YouTube, tente --cookies-from-browser chrome/firefox/safari."
fi
