use crate::components::platform::binary_functions::BinaryFunction;
use crate::components::platform::bus::Bus;
use crate::components::platform::wire::Wire;
use crate::components::{
    Component, IOComponent, UnaryBusOutputComponent, UnaryWireOutputComponent,
};
use crate::types::{Trit, WORD_SIZE};

pub struct BinaryGate {
    name: String,
    bus1: Bus,
    bus2: Bus,
    func: BinaryFunction,
    out: Bus,
}

impl BinaryGate {
    pub fn new(name: String, bus1: Bus, bus2: Bus, func: BinaryFunction) -> Self {
        BinaryGate {
            name,
            bus1,
            bus2,
            func,
            out: Bus::new(),
        }
    }
}

impl Component for BinaryGate {
    fn update(&mut self) {
        for i in 0..WORD_SIZE {
            let a = self.bus1.read_trit(i);
            let b = self.bus2.read_trit(i);
            self.out.write_trit(i, (self.func)(&a, &b));
        }
    }
}

impl UnaryBusOutputComponent for BinaryGate {
    fn o_bus1(&self) -> &Bus {
        &self.out
    }
}

impl std::fmt::Debug for BinaryGate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BinaryGate {{ name: {}, bus1: {:?}, bus2: {:?}, out: {:?} }}",
            self.name,
            self.bus1.read_word(),
            self.bus2.read_word(),
            self.out.read_word()
        )
    }
}

pub struct BinaryTritGate {
    name: String,
    wire1: Wire,
    wire2: Wire,
    func: BinaryFunction,
    out: Wire,
}

impl BinaryTritGate {
    pub fn new(name: String, wire1: Wire, wire2: Wire, func: BinaryFunction) -> Self {
        BinaryTritGate {
            name,
            wire1,
            wire2,
            func,
            out: Wire::new(Trit::default()),
        }
    }
}

impl Component for BinaryTritGate {
    fn update(&mut self) {
        self.out
            .write(&(self.func)(&self.wire1.read(), &self.wire2.read()));
    }
}

impl UnaryWireOutputComponent for BinaryTritGate {
    fn o_wire1(&self) -> &Wire {
        &self.out
    }
}

impl std::fmt::Debug for BinaryTritGate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BinaryTritGate {{ name: {}, wire1: {:?}, wire2: {:?}, out: {:?} }}",
            self.name, self.wire1, self.wire2, self.out
        )
    }
}
