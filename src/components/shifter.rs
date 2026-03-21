use crate::components::platform::bus::Bus;
use crate::components::platform::mux::TritMux;
use crate::components::platform::wire::Wire;
use crate::components::{Component, UnaryBusOutputComponent, UnaryWireOutputComponent};
use crate::types::WORD_SIZE;
use std::fmt::Debug;

/// Shifter component for a word.
/// It takes an input bus and control wires to determine the shift amount and direction.
/// The output is the shifted word on the output bus.
/// - bus1: input word (word aligned)
/// - c1, c2, c3: control wires for shift amount (0 to 8)
/// - c4: control wire for shift direction (0 for left shift, 1 for right shift)
pub struct Shifter {
    bus1: Bus,

    c1: Wire,
    c2: Wire,
    c3: Wire,

    out: Bus,

    pre_inv: [TritMux; WORD_SIZE],
    stage1: [TritMux; WORD_SIZE],
    stage2: [TritMux; WORD_SIZE],
    post_inv: [TritMux; WORD_SIZE],
}

/// Trit shifter for a word. Supports shifts from 0 to 8 positions, with the shift amount determined by control wires c1, c2, and c3.
/// c4 is used for pre- and post-inversion to enable both left and right shifts. If c4 is 0, the shifter performs a left shift; if c4 is 1, it performs a right shift.
impl Shifter {
    //noinspection DuplicatedCode
    pub fn new(bus1: Bus, c1: Wire, c2: Wire, c3: Wire) -> Self {
        let mut pre_inv = Vec::new();
        let mut post_inv = Vec::new();
        let mut stage1 = Vec::new();
        let mut stage2 = Vec::new();

        let _zero_wire = Wire::new(Default::default()).clone();

        // pre-inverter
        for i in 0..WORD_SIZE {
            pre_inv.push(TritMux::new(
                vec![c3.clone()],
                vec![
                    bus1.get_wire(i).clone(),
                    bus1.get_wire(WORD_SIZE - 1 - i).clone(),
                ]
            ));
        }

        // shifter
        macro_rules! link_shift_wires {
            ($i:expr, $muxes:expr, $shift:expr) => {
                if $i < $shift {
                    _zero_wire.clone()
                } else {
                    $muxes[$i - $shift].o_wire1().clone()
                }
            };
        }
        for i in 0..WORD_SIZE {
            stage1.push(TritMux::new(
                vec![c1.clone()],
                vec![
                    pre_inv[i].o_wire1().clone(),
                    link_shift_wires!(i, pre_inv, 1),
                    link_shift_wires!(i, pre_inv, 2),
                ]
            ));
            stage2.push(TritMux::new(
                vec![c2.clone()],
                vec![
                    stage1[i].o_wire1().clone(),
                    link_shift_wires!(i, stage1, 3),
                    link_shift_wires!(i, stage1, 6),
                ],
            ));
        }

        // post-inverter
        for i in 0..WORD_SIZE {
            post_inv.push(TritMux::new(
                vec![c3.clone()],
                vec![
                    stage2[i].o_wire1().clone(),
                    stage2[WORD_SIZE - 1 - i].o_wire1().clone(),
                ]
            ));
        }
        
        let out = Bus::from_wires(
            post_inv
                .iter()
                .map(|mux| mux.o_wire1().clone())
                .collect::<Vec<Wire>>()
                .try_into()
                .expect("Invalid wire count for bus"),
        );

        Shifter {
            bus1,
            c1,
            c2,
            c3,
            out,
            pre_inv: pre_inv.try_into().unwrap(),
            stage1: stage1.try_into().unwrap(),
            stage2: stage2.try_into().unwrap(),
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
        for mux in self.post_inv.iter_mut() {
            mux.update();
        }
    }
}

impl Debug for Shifter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Shifter {{ stage1: {:?}, stage2: {:?} }}",
            self.stage1
                .iter()
                .map(|mux| mux.o_wire1().clone())
                .collect::<Vec<Wire>>(),
            self.stage2
                .iter()
                .map(|mux| mux.o_wire1().clone())
                .collect::<Vec<Wire>>()
        )
    }
}

impl UnaryBusOutputComponent for Shifter {
    fn o_bus1(&self) -> &Bus {
        &self.out
    }
}

#[cfg(test)]
mod tests {
    use crate::components::platform::bus::Bus;
    use crate::components::shifter::Shifter;
    use crate::components::platform::wire::{wire, write};
    use crate::components::{Component, UnaryBusOutputComponent};
    use crate::conversions::{convert_int_to_unsigned_word, convert_int_to_word};
    use crate::types::{Trit, Word, WORD_SIZE};
    use log::debug;

    #[test]
    fn test_lshift() {
        pretty_env_logger::try_init().ok();
        let bus1 = Bus::from_word(&Word::state([1, 0, 0, 0, 0, 0, 0, 0, 0]));
        let c1 = wire(Trit::default());
        let c2 = wire(Trit::default());
        let c3 = wire(Trit::N);

        let mut shifter = Shifter::new(bus1, c1.clone(), c2.clone(), c3.clone());

        for i in 0..WORD_SIZE {
            let w = convert_int_to_unsigned_word(i as i32);
            write(&c1, w.get_trit(0));
            write(&c2, w.get_trit(1));
            shifter.update();

            let expected = 3_i32.pow(i as u32);
            debug!("{:?}", shifter.stage1);
            debug!("{:?}", shifter.stage2);
            assert_eq!(
                shifter.o_bus1().read_word(),
                convert_int_to_word(expected),
                "Failed at shift {}",
                i
            );
        }
    }

    #[test]
    fn test_rshift() {
        pretty_env_logger::try_init().ok();
        let bus1 = Bus::from_word(&Word::state([0, 0, 0, 0, 0, 0, 0, 0, 1]));
        let c1 = wire(Trit::default());
        let c2 = wire(Trit::default());
        let c3 = wire(Trit::Z);

        let mut shifter = Shifter::new(bus1, c1.clone(), c2.clone(), c3.clone());

        for i in 0..WORD_SIZE {
            let w = convert_int_to_unsigned_word(i as i32);
            write(&c1, w.get_trit(0));
            write(&c2, w.get_trit(1));
            shifter.update();

            let expected = 3_i32.pow((WORD_SIZE - 1 - i) as u32);
            debug!("{:?}", shifter.stage1);
            debug!("{:?}", shifter.stage2);
            assert_eq!(
                shifter.o_bus1().read_word(),
                convert_int_to_word(expected),
                "Failed at shift {}",
                i
            );
        }
    }
}
