use super::Input;

pub struct Signal(u16);

impl Signal {
    #[inline]
    pub fn new(value: u16) -> Self {
        Self(value)
    }
}

impl Input for Signal {
    #[inline]
    fn output(&self) -> Option<u16> {
        Some(self.0)
    }

    #[inline]
    fn dbg_string(&self) -> String {
        format!("Signal({})", self.0)
    }
}
