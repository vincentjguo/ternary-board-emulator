use std::fmt::Debug;
use crate::components::adder::Adder;
use crate::components::bus::Bus;
use crate::components::negate::Negate;
use crate::components::wire::{Wire, wire, read};
use crate::components::{
    BinaryWireOutputComponent, Component, UnaryBusOutputComponent, UnaryWireOutputComponent,
};
use crate::types::{Trit, WORD_SIZE};

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
        let negate: [Negate; WORD_SIZE] = std::array::from_fn(|i| {
            Negate::new(bus2.get_wire(i).clone(), control.clone())
        });
        let mut carry = wire(Trit::default());
        let adder: [Adder; WORD_SIZE] = std::array::from_fn(|i| {
            let adder = Adder::new(
                bus1.get_wire(i).clone(),
                negate[i].o_wire1().clone(),
                carry.clone(),
            );
            carry = adder.o_wire1().clone();
            adder
        });
        let s_out = Bus::from_wires(
            adder
                .iter()
                .map(|a| a.o_wire2().clone())
                .collect::<Vec<Wire>>()
                .try_into()
                .expect("Invalid wire count for bus"),
        );

        let c_out = adder[WORD_SIZE - 1].o_wire1().clone();
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
        for negate in self.negate.iter_mut() {
            negate.update();
        }

        for adder in self.adder.iter_mut() {
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
        let a = self.bus1.read_word();
        let b = self.bus2.read_word();
        let control = read(&self.control);
        let sum = self.s_out.read_word();
        let c_out = read(&self.c_out);
        write!(
            f,
            "AddSub {{ bus1: {:?}, bus2: {:?}, control: {:?}, sum_out: {:?}, c_out: {:?} }}",
            a, b, control, sum, c_out
        )
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::{max, min};
    use log::{debug};

    use super::*;
    use crate::components::wire::{read};
    use crate::tools;
    use crate::types::{Word, MAX_VALUE, MIN_VALUE};

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

        // Create and update ALU
        let mut add_sub = AddSub::new(bus1, bus2, control);
        add_sub.update();

        // Get outputs
        let result_sum = add_sub.o_bus1().read_word();
        let result_overflow = read(add_sub.o_wire1());

        debug!(
            "{}: {} + {} with control {:?} => sum: {:?}, overflow: {:?}",
            test_case.name,
            tools::convert_word_to_int(&test_case.a),
            tools::convert_word_to_int(&test_case.b),
            test_case.control,
            tools::convert_word_to_int(&result_sum),
            result_overflow
        );

        // Assert results
        assert_eq!(
            result_overflow, test_case.expected_overflow,
            "{}: Expected overflow {:?}, got {:?}",
            test_case.name, test_case.expected_overflow, result_overflow
        );
        if test_case.expected_overflow == Trit::Z {
            assert_eq!(
                result_sum, test_case.expected_sum,
                "{}: Expected sum {:?}, got {:?}",
                test_case.name, test_case.expected_sum, result_sum
            );
        }

        debug!("=> OK")
    }

    #[test]
    fn test_addsub_addition() {
        let mut test_cases = vec![];

        // Generate test cases for addition with various combinations of a and b
        // We limit j to be within the min-max range so we don't generate the same overflow cases
        // We allow 2 overflow cases on either neg or pos side
        for i in [MIN_VALUE, MAX_VALUE, 0, -1, 1] {
            for j in max(MIN_VALUE, MIN_VALUE+i+1)..min(MAX_VALUE, MAX_VALUE+i+1) {
                let a = tools::convert_int_to_word(i);
                let b = tools::convert_int_to_word(j);
                let expected_sum = tools::convert_int_to_word((i + j).clamp(MIN_VALUE, MAX_VALUE));
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
            for j in max(MIN_VALUE, MIN_VALUE+i+1)..min(MAX_VALUE, MAX_VALUE+i+1) {
                let a = tools::convert_int_to_word(i);
                let b = tools::convert_int_to_word(j);
                let expected_sum = tools::convert_int_to_word((i - j).clamp(MIN_VALUE, MAX_VALUE));
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
            let a = tools::convert_int_to_word(i);
            let b = tools::convert_int_to_word(1); // b can be anything in passthrough mode
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
