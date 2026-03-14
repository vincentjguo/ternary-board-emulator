use crate::components::binary_gate::BinaryGate;
use crate::components::bus::Bus;
use crate::components::mux::Mux;
use crate::components::wire::Wire;
use crate::gates::{and, any, cons, or, add, mul};
use crate::types::Trit;

pub struct FBlock {
    in1: Bus,
    in2: Bus,
    out: Bus,
    select: [Wire; 2],
    mux: Mux,
    functions: Vec<BinaryGate>
}

const FUNCTIONS: [fn(&Trit, &Trit) -> Trit; 6] = [
    and,
    or,
    cons,
    any,
    add,
    mul
];

impl FBlock {
    pub fn new(in1: Bus, in2: Bus, out: Bus, select: [Wire; 2]) -> Self {
        let mut functions = Vec::new();
        let mut mux_inputs = Vec::new();
        for func in FUNCTIONS.iter() {
            let output = Bus::new();
            let gate = BinaryGate::new(
                format!("{:?}", func),
                in1.clone(),
                in2.clone(),
                Box::new(*func),
                output.clone()
            );
            functions.push(gate);
            mux_inputs.push(output);
        }
        
        FBlock{
            in1,
            in2,
            out: out.clone(),
            mux: Mux::new(select.to_vec(), mux_inputs, out.clone()),
            select,
            functions
        }
    }
}



