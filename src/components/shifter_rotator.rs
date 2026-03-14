use crate::components::bus::Bus;
use crate::components::mux::Mux;
use crate::components::wire::Wire;
use crate::types::WORD_SIZE;

struct RShift {
    bus1: Bus,

    c1: Wire,
    c2: Wire,
    c3: Wire,

    out: Bus,

    stage1: [Mux; WORD_SIZE],
    stage2: [Mux; WORD_SIZE],
    stage3: [Mux; WORD_SIZE]
}

impl RShift {
    
    pub fn new(bus1: Bus, c1: Wire, c2: Wire, c3: Wire) -> Self {
        let out = Bus::new();
        let mut stage1 = Vec::new();
        let mut stage2 = Vec::new();
        let mut stage3 = Vec::new();

        for i in 0..WORD_SIZE {
            stage1.push(Mux::new(vec![c1.clone()], vec![bus1.get_wire(i), bus1.get_wire((i + 1) % WORD_SIZE)], out.get_wire(i).clone()));
            stage2.push(Mux::new(vec![c2.clone()], vec![out.get_wire(i), out.get_wire((i + 2) % WORD_SIZE)], out.get_wire(i).clone()));
            stage3.push(Mux::new(vec![c3.clone()], vec![out.get_wire(i), out.get_wire((i + 4) % WORD_SIZE)], out.get_wire(i).clone()));
        }

        RShift { bus1, c1, c2, c3, out, stage1: stage1.try_into().unwrap(), stage2: stage2.try_into().unwrap(), stage3: stage3.try_into().unwrap() }
    }
}