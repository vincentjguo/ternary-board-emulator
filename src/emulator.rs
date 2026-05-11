use crate::components::adder::Adder;
use crate::components::alu::ALU;
use crate::components::{BinaryWireOutputComponent, UnaryBusOutputComponent};
use crate::components::opcode_decoder::OpcodeDecoder;
use crate::components::platform::binary_gate::BinaryGate;
use crate::components::platform::bus::Bus;
use crate::components::platform::memory::Memory;
use crate::components::platform::mux::{BinaryTritMux, Mux};
use crate::components::platform::wire::{wire, Wire};
use crate::components::registers::register_file::RegisterFile;
use crate::components::shifter::Shifter;
use crate::types::Trit;

pub struct Emulator {
    // memory
    memory: Memory,
    write_reg_mux: Mux,

    // register
    register: RegisterFile,
    reg_dst_mux: BinaryTritMux,

    // control
    control: OpcodeDecoder,

    // alu
    alu: ALU,
    alu_src_mux: Mux,

    pc_inc: Adder,
    branch_inc: Adder,
    branch_shift: Shifter,
    branch_and_gate: BinaryGate
}

impl Emulator {
        pub fn new() -> Self {
            let temp_mem_out = Bus::new();
            let temp_alu_out = Bus::new();

            let control = OpcodeDecoder::new(temp_mem_out.clone());

            let write_reg_mux = Mux::new(
                vec![control.reg_write],
                vec![
                    temp_mem_out.clone(),
                    Bus::new(),
                    temp_alu_out.clone(),
                ]
            );

            let reg_dst_mux = BinaryTritMux::new(
                vec![control.reg_dst],
                vec![
                    [temp_mem_out[5].clone(), temp_mem_out[6].clone()],
                    [wire(Trit::Z), wire(Trit::Z)],
                    [temp_mem_out[7].clone(), temp_mem_out[8].clone()],
                ],
            );
            let register = RegisterFile::new(
                [temp_mem_out[3].clone(), temp_mem_out[4].clone()],
                [temp_mem_out[5].clone(), temp_mem_out[6].clone()],
                [reg_dst_mux.o_wire1().clone(), reg_dst_mux.o_wire2().clone()],
                control.reg_write,
                write_reg_mux.o_bus1().clone()
            );

            let alu = ALU::new();
            let alu_src_mux = Mux::new(
                vec![],
                vec![],
            );
            let pc_inc = Adder::new();
            let branch_inc = Adder::new();
            let branch_shift = Shifter::new();
            let branch_and_gate = BinaryGate::new();



            let memory = Memory::new(control.mem_control, );
            Emulator {
                memory,
                write_reg_mux,
                register,
                reg_dst_mux,
                control,
                alu,
                alu_src_mux,
                pc_inc,
                branch_inc,
                branch_shift,
                branch_and_gate
            }
        }
}