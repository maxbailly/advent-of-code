use std::cell::RefCell;
use std::fmt::{Debug, Formatter, Result};
use std::rc::Rc;

use super::Wire;
use crate::Input;

/* ---------- */

pub struct WireRef(Rc<RefCell<Wire>>);

impl WireRef {
    #[inline]
    pub fn new(id: &'static str) -> Self {
        Self(Rc::new(RefCell::new(Wire::new(id))))
    }

    #[inline]
    pub fn id(&self) -> &'static str {
        (*self.0).borrow().id()
    }

    #[inline]
    pub fn plug(&self, input: Box<dyn Input>) {
        (*self.0).borrow_mut().plug(input)
    }

    #[inline]
    pub fn reset(&self) {
        (*self.0).borrow_mut().reset()
    }
}

impl Debug for WireRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let inner = (*self.0).borrow();
        write!(f, "{inner:?}")
    }
}

impl Clone for WireRef {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl Input for WireRef {
    fn output(&self) -> Option<u16> {
        (*self.0).borrow_mut().output()
    }

    fn dbg_string(&self) -> String {
        format!("Wire({})", self.id())
    }
}
