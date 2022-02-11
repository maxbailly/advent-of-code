use crate::Input;

/* ---------- */

pub struct Not {
    input: Box<dyn Input>
}

impl Not {
    #[inline]
    pub fn new(input: Box<dyn Input>) -> Self {
        Self { input }
    }
}

impl Input for Not {
    #[inline]
    fn output(&self) -> Option<u16> {
        self.input.output().map(|val| !val)
    }

    #[inline]
    fn dbg_string(&self) -> String {
        format!("NotGate({})", self.input.dbg_string())
    }
}
