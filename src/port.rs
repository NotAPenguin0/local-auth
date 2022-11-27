pub struct Port(u16);

impl Into<u16> for Port {
    fn into(self) -> u16 {
        self.0
    }
}