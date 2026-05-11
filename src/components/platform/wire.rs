use crate::components::IOComponent;
use crate::types::Trit;
use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Clone)]
pub struct Wire {
    id: u64,

    ref_trit: Rc<RefCell<Trit>>
}

impl Wire {
    pub fn new(trit: Trit) -> Wire {
        static COUNTER: AtomicU64 = AtomicU64::new(0);

        let id = COUNTER.fetch_add(1, Ordering::SeqCst);

        Wire{
            id,
            ref_trit: Rc::new(RefCell::new(trit))
        }
    }
    pub fn default() -> Wire {
        wire(Trit::Z)
    }
}

impl IOComponent<Trit> for Wire {
    fn read(&mut self) -> Trit {
        read(self)
    }
    fn write(&mut self, value: &Trit) {
        write(self, value);
    }
}

impl Debug for Wire {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{Wire({}): {:?}}}", self.id, read(self))
    }
}

pub fn wire(value: Trit) -> Wire {
    Wire::new(value)
}

pub fn read(w: &Wire) -> Trit {
    *w.ref_trit.borrow()
}

pub fn write(w: &Wire, v: &Trit) {
    (*w.ref_trit.borrow_mut()).set_state(v);
}
