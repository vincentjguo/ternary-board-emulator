use crate::components::bus::Bus;
use crate::components::{Component, UnaryBusOutputComponent};
use crate::components::mux::{Mux, TritMux};
use crate::components::wire::Wire;
use crate::types::WORD_SIZE;

struct Shifter {
    bus1: Bus,

    c1: Wire,
    c2: Wire,
    c3: Wire,
    c4: Wire,

    out: Bus,

    pre_inv: [TritMux; WORD_SIZE],
    stage1: [TritMux; WORD_SIZE],
    stage2: [TritMux; WORD_SIZE],
    stage3: [TritMux; WORD_SIZE],
    post_inv: [TritMux; WORD_SIZE],
}

/// Trit shifter for a word. Supports shifts from 0 to 8 positions, with the shift amount determined by control wires c1, c2, and c3.
/// c4 is used for pre- and post-inversion to enable both left and right shifts. If c4 is 0, the shifter performs a left shift; if c4 is 1, it performs a right shift.
impl Shifter {
    pub fn new(bus1: Bus, c1: Wire, c2: Wire, c3: Wire, c4: Wire) -> Self {
        let pre_invert_out = Bus::new();
        let stage1_out = Bus::new();
        let stage2_out = Bus::new();
        let stage3_out = Bus::new();
        let out = Bus::new();
        let mut pre_inv = Vec::new();
        let mut post_inv = Vec::new();
        let mut stage1 = Vec::new();
        let mut stage2 = Vec::new();
        let mut stage3 = Vec::new();

        // pre-inverter
        for i in 0..WORD_SIZE {
            pre_inv.push(TritMux::new(
                vec![c4.clone()],
                vec![
                    bus1.get_wire(i).clone(),
                    bus1.get_wire(WORD_SIZE - 1 - i).clone()
                ],
                pre_invert_out.get_wire(i).clone(),
            ));
        }

        // shifter
        macro_rules! link_shift_wires {
            ($i:expr, $bus:expr, $shift:expr) => {
                if $i < $shift {
                    Wire::new(Default::default()).clone()
                } else {
                    $bus.get_wire($i - $shift).clone()
                }
            };
        }
        for i in 0..WORD_SIZE {
            stage1.push(TritMux::new(
                vec![c1.clone()],
                vec![
                    pre_invert_out.get_wire(i).clone(),
                    link_shift_wires!(i, pre_invert_out, 1),
                    link_shift_wires!(i, pre_invert_out, 2),
                ],
                stage1_out.get_wire(i).clone(),
            ));
            stage2.push(TritMux::new(
                vec![c2.clone()],
                vec![
                    stage1_out.get_wire(i).clone(),
                    link_shift_wires!(i, stage1_out, 3),
                    link_shift_wires!(i, stage1_out, 6),
                ],
                stage2_out.get_wire(i).clone(),
            ));
            stage3.push(TritMux::new(
                vec![c3.clone()],
                vec![
                    stage2_out.get_wire(i).clone(),
                    link_shift_wires!(i, stage2_out, 7),
                    link_shift_wires!(i, stage2_out, 8),
                ],
                stage3_out.get_wire(i).clone(),
            ));
        }

        // post-inverter
        for i in 0..WORD_SIZE {
            post_inv.push(TritMux::new(
                vec![c4.clone()],
                vec![
                    stage3_out.get_wire(i).clone(),
                    stage3_out.get_wire(WORD_SIZE - 1 - i).clone()
                ],
                out.get_wire(i).clone(),
            ));
        }

        Shifter {
            bus1,
            c1,
            c2,
            c3,
            c4,
            out,
            pre_inv: pre_inv.try_into().unwrap(),
            stage1: stage1.try_into().unwrap(),
            stage2: stage2.try_into().unwrap(),
            stage3: stage3.try_into().unwrap(),
            post_inv: post_inv.try_into().unwrap(),
        }
    }
}

impl Component for Shifter {
    fn update(&mut self) {
        for mux in self.pre_inv.iter_mut() {
            mux.update();
        }
        for mux in self.stage1.iter_mut() {
            mux.update();
        }
        for mux in self.stage2.iter_mut() {
            mux.update();
        }
        for mux in self.stage3.iter_mut() {
            mux.update();
        }
        for mux in self.post_inv.iter_mut() {
            mux.update();
        }
    }
}

impl UnaryBusOutputComponent for Shifter {
    fn o_bus1(&self) -> &Bus {
        &self.out
    }
}

mod tests {
    use log::info;
    use crate::components::bus::Bus;
    use crate::components::shifter_rotator::Shifter;
    use crate::components::{Component, UnaryBusOutputComponent};
    use crate::components::wire::{wire, write, Wire};
    use crate::tools::{convert_int_to_unsigned_word, convert_int_to_word};
    use crate::types::{Trit, Word, MAX_VALUE, WORD_SIZE};

    #[test]
    fn test_lshift() {
        pretty_env_logger::init();
        let bus1 = Bus::from_word(&Word::state([1, 0, 0, 0, 0, 0, 0, 0, 0]));
        let c1 = wire(Trit::default());
        let c2 = wire(Trit::default());
        let c3 = wire(Trit::default());
        let c4 = wire(Trit::N);

        let mut shifter = Shifter::new(
            bus1, c1.clone(), c2.clone(), c3.clone(), c4.clone()
        );

        for i in 0..WORD_SIZE {
            let w = convert_int_to_unsigned_word(i as i32);
            write(&c1, w.get_trit(0));
            write(&c2, w.get_trit(1));
            write(&c3, w.get_trit(2));
            shifter.update();

            let expected = 3_i32.pow(i as u32);
            info!("{:?}", shifter.stage1);
            info!("{:?}", shifter.stage2);
            info!("{:?}", shifter.stage3);
            assert_eq!(shifter.o_bus1().read_word(), convert_int_to_word(expected), "Failed at shift {}", i);
        }
    }
}
