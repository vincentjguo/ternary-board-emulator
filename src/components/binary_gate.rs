use crate::components::bus::Bus;
use crate::components::{Component, UnaryBusOutputComponent};
use crate::types::{Trit, WORD_SIZE};

pub struct BinaryGate {
    name: String,
    bus1: Bus,
    bus2: Bus,
    func: Box<dyn Fn(&Trit, &Trit) -> Trit>,
    out: Bus
}

impl BinaryGate {
    pub fn new(name: String, bus1: Bus, bus2: Bus, func: Box<dyn Fn(&Trit, &Trit) -> Trit>, out: Bus) -> Self {
        BinaryGate { name, bus1, bus2, func, out }
    }
}

impl Component for BinaryGate {
    fn update(&mut self) {
        for i in 0..WORD_SIZE {
            let a = self.bus1.read_trit(i);
            let b = self.bus2.read_trit(i);
            self.out.write_trit((self.func)(&a, &b), i);
        }
    }
}

impl UnaryBusOutputComponent for BinaryGate {
    fn o_bus1(&self) -> &Bus {
        &self.out
    }
}



