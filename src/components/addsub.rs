use crate::components::adder::Adder;
use crate::components::negate::Negate;
use crate::components::platform::bus::Bus;
use crate::components::platform::wire::{Wire, read, wire};
use crate::components::{
    BinaryWireOutputComponent, Component, UnaryBusOutputComponent, UnaryWireOutputComponent,
};
use crate::types::{Trit, WORD_SIZE};
use std::fmt::{Debug, Display, Formatter};

/// ALU component.
/// - bus1: a (word aligned)
/// - bus2: b (word aligned)
/// - control: if 1, out = in1 + in2; if -1, out = in1 - in2; if 0, out = in1
/// - c_out (out): carry out/overflow
/// - s_out (bus): sum output
pub struct AddSub {
    bus1: Bus,
    bus2: Bus,
    control: Wire,
    negate: [Negate; WORD_SIZE],
    adder: [Adder; WORD_SIZE],

    s_out: Bus,
    c_out: Wire,
}

impl AddSub {
    pub fn new(bus1: Bus, bus2: Bus, control: Wire) -> Self {
        let negate: [Negate; WORD_SIZE] =
            std::array::from_fn(|i| Negate::new(bus2.get_wire(i).clone(), control.clone()));
        let mut carry = wire(Trit::default());
        let mut adder: [Adder; WORD_SIZE] = std::array::from_fn(|i| {
            let reversed_i = WORD_SIZE - 1 - i;
            let adder = Adder::new(
                bus1.get_wire(reversed_i).clone(),
                negate[reversed_i].o_wire1().clone(),
                carry.clone(),
            );
            carry = adder.o_wire1().clone();
            adder
        });
        adder.reverse();
        let s_out = Bus::from_wires(
            adder
                .iter()
                .map(|a| a.o_wire2().clone())
                .collect::<Vec<Wire>>()
                .try_into()
                .expect("Invalid wire count for bus"),
        );

        let c_out = adder[0].o_wire1().clone();
        AddSub {
            bus1,
            bus2,
            control,
            negate,
            adder,
            s_out,
            c_out,
        }
    }
}

impl Component for AddSub {
    fn update(&mut self) {
        for negate in self.negate.iter_mut().rev() {
            negate.update();
        }

        for adder in self.adder.iter_mut().rev() {
            adder.update();
        }
    }
}

impl UnaryWireOutputComponent for AddSub {
    fn o_wire1(&self) -> &Wire {
        &self.c_out
    }
}

impl UnaryBusOutputComponent for AddSub {
    fn o_bus1(&self) -> &Bus {
        &self.s_out
    }
}

impl Debug for AddSub {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AddSub {{ bus1: {:?}\n bus2: {:?}\n control: {:?}\n sum_out: {:?}\n c_out: {:?}\n adders: {:?}}}",
            self.bus1, self.bus2, self.control, self.s_out, self.c_out, self.adder
        )
    }
}

#[cfg(test)]
mod tests {
    use log::debug;
    use std::cmp::{max, min};

    use super::*;
    use crate::components::platform::wire::read;
    use crate::conversions;
    use crate::types::{MAX_VALUE, MIN_VALUE, Word};

    struct AddSubTestCase {
        name: String,
        a: Word,
        b: Word,
        control: Trit,
        expected_sum: Word,
        expected_overflow: Trit,
    }

    fn run_addsub_test(test_case: AddSubTestCase) {
        pretty_env_logger::try_init().ok();
        // Create input buses and control wire
        let bus1 = Bus::from_word(&test_case.a);
        let bus2 = Bus::from_word(&test_case.b);
        let control = wire(test_case.control);

        // Create and update AddSub component
        let mut add_sub = AddSub::new(bus1, bus2, control);
        add_sub.update();

        // Get outputs
        let result_sum = add_sub.o_bus1().read_word();
        let result_overflow = read(add_sub.o_wire1());

        debug!(
            "{} with control {:?} => sum: {:?}, overflow: {:?}",
            test_case.name,
            test_case.control,
            conversions::convert_word_to_int(&result_sum),
            result_overflow
        );

        // Assert results
        assert_eq!(
            result_overflow, test_case.expected_overflow,
            "{}: Expected overflow {:?}, got {:?}\n {:?}",
            test_case.name, test_case.expected_overflow, result_overflow, add_sub
        );
        if test_case.expected_overflow == Trit::Z {
            assert_eq!(
                result_sum, test_case.expected_sum,
                "{}: Expected sum {:?}, got {:?}\n {:?}",
                test_case.name, test_case.expected_sum, result_sum, add_sub
            );
        }

        debug!("=> OK")
    }

    // #[test]
    // #[ignore]
    // fn test_specific() {
    //     let mut test_cases = vec![
    //         AddSubTestCase {
    //             name: format!("Addition: {} + {}", -9841, -3280),
    //             a: conversions::convert_int_to_word(-9841),
    //             b: conversions::convert_int_to_word(-3280),
    //             control: Trit::P,
    //             expected_sum: Word::new(),
    //             expected_overflow: Trit::N,
    //         }
    //     ];
    //
    //     for test_case in test_cases {
    //         run_addsub_test(test_case);
    //     }
    // }

    #[test]
    fn test_addsub_addition() {
        let mut test_cases = vec![];

        // Generate test cases for addition with various combinations of a and b
        // We limit j to be within the min-max range so we don't generate the same overflow cases
        // We allow 2 overflow cases on either neg or pos side
        for i in [MIN_VALUE, MAX_VALUE, 0, -1, 1] {
            for j in max(MIN_VALUE, MIN_VALUE + i + 1)..min(MAX_VALUE, MAX_VALUE + i + 1) {
                let a = conversions::convert_int_to_word(i);
                let b = conversions::convert_int_to_word(j);
                let expected_sum =
                    conversions::convert_int_to_word((i + j).clamp(MIN_VALUE, MAX_VALUE));
                let expected_overflow = if i + j < MIN_VALUE {
                    Trit::N
                } else if i + j > MAX_VALUE {
                    Trit::P
                } else {
                    Trit::Z
                };
                test_cases.push(AddSubTestCase {
                    name: format!("Addition: {} + {}", i, j),
                    a,
                    b,
                    control: Trit::P,
                    expected_sum,
                    expected_overflow,
                });
            }
        }

        for test_case in test_cases {
            run_addsub_test(test_case);
        }
    }

    #[test]
    fn test_addsub_subtraction() {
        let mut test_cases = vec![];

        for i in [MIN_VALUE, MAX_VALUE, 0, -1, 1] {
            for j in max(MIN_VALUE, MIN_VALUE + i + 1)..min(MAX_VALUE, MAX_VALUE + i + 1) {
                let a = conversions::convert_int_to_word(i);
                let b = conversions::convert_int_to_word(j);
                let expected_sum =
                    conversions::convert_int_to_word((i - j).clamp(MIN_VALUE, MAX_VALUE));
                let expected_overflow = if i - j < MIN_VALUE {
                    Trit::N
                } else if i - j > MAX_VALUE {
                    Trit::P
                } else {
                    Trit::Z
                };
                test_cases.push(AddSubTestCase {
                    name: format!("Subtraction: {} - {}", i, j),
                    a,
                    b,
                    control: Trit::N,
                    expected_sum,
                    expected_overflow,
                });
            }
        }

        for test_case in test_cases {
            run_addsub_test(test_case);
        }
    }

    #[test]
    fn test_addsub_passthrough() {
        let mut test_cases = vec![];

        for i in MIN_VALUE..MAX_VALUE {
            let a = conversions::convert_int_to_word(i);
            let b = conversions::convert_int_to_word(1); // b can be anything in passthrough mode
            let expected_sum = a.clone();
            test_cases.push(AddSubTestCase {
                name: format!("Passthrough: {}", i),
                a,
                b, // b is ignored in passthrough mode
                control: Trit::Z,
                expected_sum,
                expected_overflow: Trit::Z, // No overflow in passthrough mode
            });
        }

        for test_case in test_cases {
            run_addsub_test(test_case);
        }
    }
}
