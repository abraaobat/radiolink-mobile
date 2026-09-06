//! Device driver integration layer.

pub struct DriverDescriptor {
    pub id: &'static str,
    pub display_name: &'static str,
}
