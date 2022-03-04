use crate::Input;

/* ---------- */

pub struct LeftShift {
    left: Box<dyn Input>,
    right: Box<dyn Input>,
}

impl LeftShift {
    #[inline]
    pub fn new(left: Box<dyn Input>, right: Box<dyn Input>) -> Self {
        Self { left, right }
    }
}

impl Input for LeftShift {
    fn output(&self) -> Option<u16> {
        let left = self.left.output();
        let right = self.right.output();

        left.zip(right).map(|(left, right)| left << right)
    }

    #[inline]
    fn dbg_string(&self) -> String {
        format!(
            "LeftShift{{left: {}, right: {}}}",
            self.left.dbg_string(),
            self.right.dbg_string()
        )
    }
}
