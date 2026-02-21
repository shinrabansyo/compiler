use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Clone, PartialEq, Eq)]
pub struct LirVar {
    inner: Rc<RefCell<Option<u32>>>,
}

impl Debug for LirVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self.inner.borrow() {
            Some(id) => write!(f, "v{}", id),
            None => write!(f, "v?"),
        }
    }
}

impl LirVar {
    pub fn new() -> Self {
        LirVar {
            inner: Rc::new(RefCell::new(None)),
        }
    }

    pub(crate) fn set(&self, id: u32) {
        assert!(self.inner.borrow().is_none(), "LirVar is already set");
        *self.inner.borrow_mut() = Some(id);
    }

    pub(crate) fn id(&self) -> u32 {
        *self.inner.borrow().as_ref().unwrap()
    }
}
