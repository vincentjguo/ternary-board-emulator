use crate::components::{Component, UnaryBusOutputComponent};
use crate::components::platform::bus::Bus;
use crate::components::platform::wire::Wire;
use crate::types::Word;

pub struct Register {
    name: String,
    d_out: Bus,
    d_in: Bus,
    control: Wire
}

impl Register {
    pub fn new(name: String, d_in: Bus, control: Wire) -> Self {

        Register {
            name,
            d_out: Bus::new(),
            d_in,
            control
        }
    }
}

impl Component for Register {
    fn update(&mut self) {
        let control = crate::components::platform::wire::read(&self.control);
        match control {
            crate::types::Trit::Z => {
                // Do nothing, keep the current value
            },
            crate::types::Trit::P => {
                // Load the new value from d_in to d_out
                let value = self.d_in.read_word();
                self.d_out.write_word(&value);
            },
            crate::types::Trit::N => {
                // Reset the register to zero
                self.d_out.write_word(&Word::new());
            }
        }
    }
}

impl UnaryBusOutputComponent for Register {
    fn o_bus1(&self) -> &Bus {
        &self.d_out
    }
}

impl std::fmt::Debug for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let d_in = self.d_in.read_word();
        let d_out = self.d_out.read_word();
        let control = crate::components::platform::wire::read(&self.control);
        write!(
            f,
            "Register {{ name: {}, d_in: {:?}, control: {:?}, d_out: {:?} }}",
            self.name, d_in, control, d_out
        )
    }
}