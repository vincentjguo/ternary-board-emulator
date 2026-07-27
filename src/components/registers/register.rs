use crate::components::platform::bus::Bus;
use crate::components::platform::wire::Wire;
use crate::components::{Component, UnaryBusOutputComponent};
use crate::types::{Trit, Word};

/// A register that can hold a value and update it based on a control signal.
/// When the control signal is non-zero, the register loads the value from d_in to d_out.
pub struct Register {
    name: String,
    d_out: Bus,
    d_in: Bus,
    control: Wire,
}

impl Register {
    pub fn new(name: String, d_in: Bus, control: Wire) -> Self {
        Register {
            name,
            d_out: Bus::new(),
            d_in,
            control,
        }
    }
}

impl Component for Register {
    fn update(&mut self) {
        let control = crate::components::platform::wire::read(&self.control);
        match control {
            Trit::Z => {
                // Do nothing, keep the current value
            }
            Trit::P | Trit::N => {
                // Load the new value from d_in to d_out
                let value = self.d_in.read_word();
                self.d_out.write_word(&value);
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

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.d_out.read_word())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::platform::bus::Bus;
    use crate::components::platform::wire::{read, wire, write};
    use crate::components::registers::register::Register;
    use crate::components::{Component, UnaryBusOutputComponent};
    use crate::types::{Trit, Word};

    #[test]
    fn test_register() {
        let d_in = Bus::new();
        let control = wire(Trit::Z);
        let mut register = Register::new("R1".to_string(), d_in.clone(), control.clone());

        // Initially, the register should hold the default value (all Z)
        assert_eq!(register.o_bus1().read_word(), Word::new());

        // Write a value to d_in and set control to P to load it into the register
        let value = Word::from_trits([Trit::P; 9]);
        d_in.write_word(&value);
        write(&control, &Trit::P);
        register.update();
        assert_eq!(register.o_bus1().read_word(), value);

        // Change control to N and update, the register should still hold the same value
        write(&control, &Trit::N);
        register.update();
        assert_eq!(register.o_bus1().read_word(), value);

        // Set control back to Z and update, the register should still hold the same value
        write(&control, &Trit::Z);
        register.update();
        assert_eq!(register.o_bus1().read_word(), value);
    }
}
