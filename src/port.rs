pub struct Port(u16);

impl From<u16> for Port {
    fn from(val: u16) -> Self {
        Port {
            0: val
        }
    }
}

impl Into<u16> for Port {
    fn into(self) -> u16 {
        self.0
    }
}