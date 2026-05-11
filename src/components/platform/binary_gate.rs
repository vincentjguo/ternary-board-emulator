use crate::components::platform::bus::Bus;
use crate::components::{Component, UnaryBusOutputComponent};
use crate::components::platform::binary_functions::BinaryFunction;
use crate::types::WORD_SIZE;

pub struct BinaryGate {
    name: String,
    bus1: Bus,
    bus2: Bus,
    func: BinaryFunction,
    out: Bus,
}

impl BinaryGate {
    pub fn new(
        name: String,
        bus1: Bus,
        bus2: Bus,
        func: BinaryFunction,
        out: Bus,
    ) -> Self {
        BinaryGate {
            name,
            bus1,
            bus2,
            func,
            out,
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