use crate::components::platform::bus::Bus;
use crate::components::registers::register_file::RegisterFile;
use crate::components::{BinaryBusOutputComponent, Component, IOComponent, UnaryBusOutputComponent};
use crate::components::platform::wire::Wire;
use crate::components::registers::PC_REGISTER_INDEX;
use crate::conversions::convert_int_to_word;
use crate::types::Trit;

/// Component to access only program counter from register file
pub struct ProgramCounter<'a> {
    register_file: &'a mut RegisterFile,
    // register file inputs
    write_enable: Wire,
    write_data: Bus,
    reg1_select: [Wire; 2],

    data_in: Bus,
    data_out: Bus,
}

impl<'a> ProgramCounter<'a> {
    pub fn new(register_file: &mut RegisterFile, write_enable: Wire, write_data: Bus, reg1_select: [Wire; 2], data_in: Bus) -> ProgramCounter {
        let data_out = register_file.o_bus1().clone();
        ProgramCounter {
            register_file,
            write_enable,
            write_data,
            reg1_select,
            data_in,
            data_out
        }
    }
}

impl<'a> Component for ProgramCounter<'a> {
    fn update(&mut self) {
        // write to the program counter register (reg 7)
        self.write_enable.write(&Trit::P);
        let pc = convert_int_to_word(PC_REGISTER_INDEX);
        self.reg1_select[0].write(&pc[7]);
        self.reg1_select[1].write(&pc[8]);
        self.write_data.write_word(&self.data_in.read_word());

        self.register_file.update();
    }
}

impl<'a> UnaryBusOutputComponent for ProgramCounter<'a> {
    fn o_bus1(&self) -> &Bus {
        &self.data_out
    }
}