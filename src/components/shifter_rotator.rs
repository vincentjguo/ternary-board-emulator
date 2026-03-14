use crate::components::bus::Bus;
use crate::components::mux::{Mux, TritMux};
use crate::components::wire::Wire;
use crate::types::WORD_SIZE;

struct LShift {
    bus1: Bus,

    c1: Wire,
    c2: Wire,
    c3: Wire,

    out: Bus,

    stage1: [TritMux; WORD_SIZE],
    stage2: [TritMux; WORD_SIZE],
    stage3: [TritMux; WORD_SIZE],
}

impl LShift {
    pub fn new(bus1: Bus, c1: Wire, c2: Wire, c3: Wire) -> Self {
        let out = Bus::new();
        let mut stage1 = Vec::new();
        let mut stage2 = Vec::new();
        let mut stage3 = Vec::new();

        let zero_wire = Wire::new(Default::default());

        macro_rules! link_shift_wires {
            ($i:expr, $bus:expr, $shift:expr) => {
                if $i < $shift {
                    zero_wire.clone()
                } else {
                    $bus.get_wire($i - $shift).clone()
                }
            };
        }
        for i in 0..WORD_SIZE {
            stage1.push(TritMux::new(
                vec![c1.clone()],
                vec![
                    bus1.get_wire(i).clone(),
                    link_shift_wires!(i, bus1, 1),
                    link_shift_wires!(i, bus1, 2),
                ],
                out.get_wire(i).clone(),
            ));
            stage2.push(TritMux::new(
                vec![c2.clone()],
                vec![
                    link_shift_wires!(i, bus1, 3),
                    link_shift_wires!(i, bus1, 4),
                    link_shift_wires!(i, bus1, 5),
                ],
                out.get_wire(i).clone(),
            ));
            stage3.push(TritMux::new(
                vec![c3.clone()],
                vec![
                    link_shift_wires!(i, bus1, 6),
                    link_shift_wires!(i, bus1, 7),
                    link_shift_wires!(i, bus1, 8),
                ],
                out.get_wire(i).clone(),
            ));
        }

        LShift {
            bus1,
            c1,
            c2,
            c3,
            out,
            stage1: stage1.try_into().unwrap(),
            stage2: stage2.try_into().unwrap(),
            stage3: stage3.try_into().unwrap(),
        }
    }
}
