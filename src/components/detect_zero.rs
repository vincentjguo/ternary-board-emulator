use crate::components::platform::binary_functions::{and, nor};
use crate::components::platform::bus::Bus;
use crate::components::platform::mux::TritMux;
use crate::components::platform::wire::{Wire, read, write};
use crate::components::{Component, UnaryWireOutputComponent};
use crate::types::Trit;

pub struct DetectZero {
    bus: Bus,
    is_zero_res: Wire,
    out: Wire,

    mux: TritMux,
}

impl DetectZero {
    pub fn new(bus: Bus) -> Self {
        let is_zero_res = Wire::new(Trit::default());

        let mux = TritMux::new(
            vec![is_zero_res.clone()],
            vec![Wire::new(Trit::N), Wire::new(Trit::P), Wire::new(Trit::N)],
        );

        Self {
            bus,
            is_zero_res,
            out: mux.o_wire1().clone(),
            mux,
        }
    }
}

impl Component for DetectZero {
    fn update(&mut self) {
        let mut is_zero = Trit::Z;
        // todo: optimize
        for i in &self.bus {
            is_zero = nor(&is_zero, &i);
        }
        for i in &self.bus {
            is_zero = and(&is_zero, &i);
        }

        write(&self.is_zero_res, &is_zero);

        self.mux.update();
        write(&self.out, &read(self.mux.o_wire1()));
    }
}

impl UnaryWireOutputComponent for DetectZero {
    fn o_wire1(&self) -> &Wire {
        &self.out
    }
}

impl std::fmt::Debug for DetectZero {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DetectZero {{ bus: {:?}, zero_res: {:?}, out: {:?} }}",
            self.bus.read_word(),
            self.is_zero_res,
            read(&self.out)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Word;

    #[test]
    fn detect_zero_test_not_zero() {
        let bus = Bus::from_word(&Word::state([1, 0, -1, 0, 1, 0, -1, 0, 0]));
        let mut component = DetectZero::new(bus.clone());

        component.update();
        assert_eq!(read(&component.out), Trit::N);

        bus.write_word(&Word::state([0, 0, 1, 1, 1, 1, 1, -1, -1]));
        assert_eq!(read(&component.out), Trit::N);

        bus.write_word(&Word::state([-1, -1, -1, -1, -1, -1, -1, -1, -1]));
        assert_eq!(read(&component.out), Trit::N);
    }

    #[test]
    fn detect_zero_test_zero() {
        let bus = Bus::from_word(&Word::state([0, 0, 0, 0, 0, 0, 0, 0, 0]));
        let mut component = DetectZero::new(bus.clone());

        component.update();

        assert_eq!(read(&component.out), Trit::P, "state: {:?}", component);
    }
}
