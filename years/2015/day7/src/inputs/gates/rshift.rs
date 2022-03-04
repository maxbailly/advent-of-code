use crate::Input;

/* ---------- */

pub struct RightShift {
    left: Box<dyn Input>,
    right: Box<dyn Input>,
}

impl RightShift {
    #[inline]
    pub fn new(left: Box<dyn Input>, right: Box<dyn Input>) -> Self {
        Self { left, right }
    }
}

impl Input for RightShift {
    fn output(&self) -> Option<u16> {
        let left = self.left.output();
        let right = self.right.output();

        left.zip(right).map(|(left, right)| left >> right)
    }

    #[inline]
    fn dbg_string(&self) -> String {
        format!(
            "RightShift{{left: {}, right: {}}}",
            self.left.dbg_string(),
            self.right.dbg_string()
        )
    }
}
