use crate::components::adder::Adder;
use crate::components::addsub::AddSub;
use crate::components::alu::ALU;
use crate::components::immediate_extend::ImmediateExtend;
use crate::components::opcode_decoder::OpcodeDecoder;
use crate::components::platform::binary_gate::BinaryGate;
use crate::components::platform::bus::Bus;
use crate::components::platform::memory::Memory;
use crate::components::platform::mux::{BinaryTritMux, LazyMux, Mux};
use crate::components::platform::wire::{Wire, wire};
use crate::components::registers::program_counter::ProgramCounter;
use crate::components::registers::register_file::RegisterFile;
use crate::components::shifter::Shifter;
use crate::components::{
    BinaryBusOutputComponent, BinaryWireOutputComponent, Component, UnaryBusOutputComponent,
};
use crate::types::Trit;

pub struct Emulator {
    // memory
    memory: Memory,
    write_data_mux: Mux,

    // register
    register: RegisterFile,
    primary_reg_mux: BinaryTritMux,
    reg_dst_mux: BinaryTritMux,

    immediate_extend: ImmediateExtend,

    // control
    control: OpcodeDecoder,

    // alu
    alu: ALU,
    alu_src_mux: Mux,

    pc_inc: Adder,
    branch_inc: Adder,
    branch_shift: Shifter,
    branch_and_gate: BinaryGate,
}

impl Emulator {
    pub fn new() -> Self {
        let temp_mem_out = Bus::new();

        let control = OpcodeDecoder::new(temp_mem_out.clone());

        let mut write_data_mux = LazyMux::new(vec![control.reg_write], vec![]);

        let primary_reg_mux = BinaryTritMux::new(
            vec![control.primary_reg],
            vec![
                [temp_mem_out[5], temp_mem_out[6]],
                [temp_mem_out[3], temp_mem_out[4]],
                [temp_mem_out[7], temp_mem_out[8]],
            ],
        );

        let reg_dst_mux = BinaryTritMux::new(
            vec![control.reg_dst],
            vec![
                [temp_mem_out[5].clone(), temp_mem_out[6].clone()],
                [wire(Trit::Z), wire(Trit::Z)],
                [temp_mem_out[7].clone(), temp_mem_out[8].clone()],
            ],
        );
        let mut register = RegisterFile::new(
            [
                primary_reg_mux.o_wire1().clone(),
                primary_reg_mux.o_wire2().clone(),
            ],
            [temp_mem_out[5].clone(), temp_mem_out[6].clone()],
            [reg_dst_mux.o_wire1().clone(), reg_dst_mux.o_wire2().clone()],
            control.reg_write.clone(),
            write_data_mux.o_bus1().clone(),
        );

        let immediate_extend =
            ImmediateExtend::new(temp_mem_out.clone(), control.jump, control.load_immediate);

        let alu_src_mux = Mux::new(
            vec![control.alu_src],
            vec![register.o_bus2().clone(), immediate_extend.o_bus1().clone()],
        );

        let alu = ALU::new(
            register.o_bus1().clone(),
            alu_src_mux.o_bus1().clone(),
            control.alu_control,
        );

        let mut memory = Memory::new(
            control.mem_control.clone(),
            register.o_bus1().clone(),
            register.o_bus2().clone(),
        );

        memory.set_data_out_bus(temp_mem_out);

        write_data_mux.set_inputs(vec![
            memory.o_bus1().clone(),
            Bus::new(),
            alu.o_bus1().clone(),
        ]);

        let pc = ProgramCounter::new(
            &mut register,
            control.reg_write.clone(),
            write_data_mux.o_bus1().clone(),
            [
                primary_reg_mux.o_wire1().clone(),
                primary_reg_mux.o_wire2().clone(),
            ],
            write_data_mux.o_bus1().clone(),
        );

        let pc_inc = AddSub::new(
            pc.o_bus1().clone(),
            Bus::from_word(&crate::types::Word::state([0, 0, 0, 0, 0, 0, 1, 0, 0])),
            Wire::new(Trit::P),
        );
        let branch_shift = Shifter::new(
            immediate_extend.o_bus1().clone(),
            Wire::new(Trit::N),
            Wire::new(Trit::P),
            Wire::new(Trit::Z),
        );
        let branch_inc = AddSub::new(
            pc_inc.o_bus1().clone(),
            branch_shift.o_bus1().clone(),
            Wire::new(Trit::P),
        );
        // TODO: change this to detect 0 on ALU
        let branch_and_gate = BinaryGate::new("Branch And", pc_inc.o_bus1().clone(), branch_inc.o_bus1().clone(), );

        Emulator {
            memory,
            write_data_mux,
            register,
            reg_dst_mux,
            control,
            alu,
            alu_src_mux,
            pc_inc,
            branch_inc,
            branch_shift,
            branch_and_gate,
        }
    }
}
