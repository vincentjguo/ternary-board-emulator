use crate::components::{platform, Component};
use crate::components::platform::wire::{read, write, Wire};
use crate::types::Trit;

/// A Decoder takes n select wires and an output signal wire, and produces 3^n output wires.
/// The output wires are all set to 0 (Z) except for the one corresponding to the selected index,
/// which is set to the value of the output signal wire.
/// The selected index is determined by interpreting the select wires as an unsigned ternary number
pub struct Decoder {
    select: Vec<Wire>,
    outputs: Vec<Wire>,
    out_sig: Wire,
    bias: i32
}

impl Decoder {
    pub fn new(select: Vec<Wire>, out_sig: Wire) -> Self {
        let outputs = (0..3_i32.pow(select.len() as u32))
            .map(|_| Wire::new(Default::default())).collect();
        let max_inputs =3usize.pow(select.len() as u32);
        Decoder {
            select,
            outputs,
            out_sig,
            bias: max_inputs as i32 /2,
        }
    }

    pub fn get_output(&self, i: usize) -> &Wire {
        &self.outputs[i]
    }
}

impl Component for Decoder {
    fn update(&mut self) {
        let idx = platform::selected_index(&self.select, self.bias);
        let out_sig_value = read(&self.out_sig);
        for (i, output) in self.outputs.iter().enumerate() {
            write(output, if i == idx { &out_sig_value } else { &Trit::Z });
        }
    }
}

impl std::fmt::Debug for Decoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let select_values: Vec<Trit> = self.select.iter().map(|wire| read(wire)).collect();
        let output_values: Vec<Trit> = self.outputs.iter().map(|wire| read(wire)).collect();
        write!(
            f,
            "Decoder {{ select: {:?}, out_sig: {:?}, outputs: {:?} }}",
            select_values,
            read(&self.out_sig),
            output_values
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::{Component};

    #[test]
    fn test_decoder() {

        let select = vec![
            crate::components::wire::wire(Trit::N),
            crate::components::wire::wire(Trit::Z),
        ];
        let out_sig = crate::components::wire::wire(Trit::P);
        let mut decoder = Decoder::new(select.clone(), out_sig.clone());

        // run through all combinations of select inputs and check outputs
        for (i, sel) in select.iter().enumerate() {
            for &trit in &[Trit::N, Trit::Z, Trit::P] {
                for &out_trit in &[Trit::N, Trit::Z, Trit::P] {
                    crate::components::wire::write(sel, &trit);
                    crate::components::wire::write(&out_sig, &out_trit);
                    decoder.update();

                    let expected_idx = platform::selected_index(&decoder.select, decoder.bias);
                    for (j, output) in decoder.outputs.iter().enumerate() {
                        let expected = if j == expected_idx { out_trit } else { Trit::Z };
                        assert_eq!(read(output), expected, "Failed at select {:?} (index {})", sel, i);
                    }
                }
            }
        }

    }
}