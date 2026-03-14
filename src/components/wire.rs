use crate::types::Trit;
use std::cell::{Ref, RefCell};
use std::rc::Rc;
use crate::components::IOComponent;

pub type Wire = Rc<RefCell<Trit>>;

impl IOComponent<Trit> for Wire {
    fn read(&mut self) -> Trit {
        read(self)
    }
    fn write(&mut self, value: &Trit) {
        write(self, value);
    }
}

pub fn wire(value: Trit) -> Wire {
    Rc::new(RefCell::new(value))
}

pub fn read(w: &Wire) -> Trit {
    *w.borrow()
}

pub fn write(w: &Wire, v: &Trit) {
    (*w.borrow_mut()).set_state(v);
}
