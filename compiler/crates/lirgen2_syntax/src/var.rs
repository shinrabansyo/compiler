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

    pub fn id(&self) -> u32 {
        *self.inner.borrow().as_ref().unwrap()
    }
}

pub struct LirVarIssuer {
    next_id: u32,
}

impl LirVarIssuer {
    pub fn new() -> Self {
        LirVarIssuer { next_id: 0 }
    }

    pub fn issue(&mut self, var: &LirVar) {
        var.inner.borrow_mut().replace(self.next_id);
        self.next_id += 1;
    }
}
