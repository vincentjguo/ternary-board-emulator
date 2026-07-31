use crate::components::addsub::AddSub;
use crate::components::alu::ALU;
use crate::components::detect_zero::DetectZero;
use crate::components::immediate_extend::ImmediateExtend;
use crate::components::opcode_decoder::OpcodeDecoder;
use crate::components::platform::binary_functions::cons;
use crate::components::platform::binary_gate::BinaryTritGate;
use crate::components::platform::bus::Bus;
use crate::components::platform::memory::Memory;
use crate::components::platform::mux::{BinaryTritMux, LazyMux, Mux};
use crate::components::platform::wire::{Wire, wire};
use crate::components::registers::register_file::RegisterFile;
use crate::components::shifter::Shifter;
use crate::components::{
    BinaryBusOutputComponent, BinaryWireOutputComponent, Component, UnaryBusOutputComponent,
    UnaryWireOutputComponent,
};
use crate::types::Trit;
use log::{debug, warn};

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
    detect_zero: DetectZero,

    // pc
    pc_inc: AddSub,

    // branch
    branch_inc: AddSub,
    branch_shift: Shifter,
    branch_cond_gate: BinaryTritGate,
    branch_select_mux: Mux,
}

impl Emulator {
    pub fn new() -> Self {
        // break the dependency cycle on instruction memory
        let temp_mem_out = Bus::new();

        let control = OpcodeDecoder::new(temp_mem_out.clone());

        let mut write_data_mux = LazyMux::new(vec![control.reg_write.clone()], vec![]);

        let primary_reg_mux = BinaryTritMux::new(
            vec![control.primary_reg.clone()],
            vec![
                [temp_mem_out[5].clone(), temp_mem_out[6].clone()],
                [temp_mem_out[3].clone(), temp_mem_out[4].clone()],
                [temp_mem_out[7].clone(), temp_mem_out[8].clone()],
            ],
        );

        let reg_dst_mux = BinaryTritMux::new(
            vec![control.reg_dst.clone()],
            vec![
                [temp_mem_out[5].clone(), temp_mem_out[6].clone()],
                [wire(Trit::Z), wire(Trit::Z)],
                [temp_mem_out[7].clone(), temp_mem_out[8].clone()],
            ],
        );
        let register = RegisterFile::new(
            [
                primary_reg_mux.o_wire1().clone(),
                primary_reg_mux.o_wire2().clone(),
            ],
            [temp_mem_out[5].clone(), temp_mem_out[6].clone()],
            [reg_dst_mux.o_wire1().clone(), reg_dst_mux.o_wire2().clone()],
            control.reg_write.clone(),
            write_data_mux.o_bus1().clone(),
        );

        let immediate_extend = ImmediateExtend::new(
            temp_mem_out.clone(),
            control.jump.clone(),
            control.load_immediate.clone(),
        );

        let alu_src_mux = Mux::new(
            vec![control.alu_src.clone()],
            vec![
                register.o_bus2().clone(),
                Bus::new(),
                immediate_extend.o_bus1().clone(),
            ],
        );

        let alu = ALU::new(
            register.o_bus1().clone(),
            alu_src_mux.o_bus1().clone(),
            control.alu_control.clone(),
        );

        let mut memory = Memory::new(
            control.mem_control.clone(),
            register.o_bus1().clone(),
            register.o_bus2().clone(),
        );
        // memory fully initialized, set the temporary bus as output
        memory.set_data_out_bus(temp_mem_out);

        write_data_mux.set_inputs(vec![
            memory.o_bus1().clone(),
            Bus::new(),
            alu.o_bus1().clone(),
        ]);

        let pc_inc = AddSub::new(
            register.o_bus1().clone(),
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
        let detect_zero = DetectZero::new(alu.o_bus1().clone());

        let branch_cond_gate = BinaryTritGate::new(
            "Branch Cond".parse().unwrap(),
            control.branch.clone(),
            detect_zero.o_wire1().clone(),
            cons,
        );

        let mut branch_select_mux: Mux = Mux::new(
            vec![branch_cond_gate.o_wire1().clone()],
            vec![
                branch_inc.o_bus1().clone(),
                pc_inc.o_bus1().clone(),
                branch_inc.o_bus1().clone(),
            ],
        );
        branch_select_mux.set_output(write_data_mux.o_bus1().clone());
        // let pc = ProgramCounter::new(
        //     &mut register,
        //     control.reg_write.clone(),
        //     write_data_mux.o_bus1().clone(),
        //     [
        //         primary_reg_mux.o_wire1().clone(),
        //         primary_reg_mux.o_wire2().clone(),
        //     ],
        //     branch_select_mux.o_bus1().clone(),
        // );

        Emulator {
            memory,
            write_data_mux,
            register,
            primary_reg_mux,
            reg_dst_mux,
            immediate_extend,
            control,
            alu,
            detect_zero,
            alu_src_mux,
            pc_inc,
            branch_inc,
            branch_shift,
            branch_cond_gate,
            branch_select_mux,
        }
    }

    pub fn execute(&mut self) {
        loop {
            self.register.update_and_read_pc();

            // fetch and decode instruction memory
            self.memory.fetch_instruction();

            if std::env::var("DEBUG").is_ok() {
                let pc_word = self.register.o_bus1().read_word();
                let instr = self.memory.o_bus1().read_word();
                debug!(
                    "cycle pc={} instr={}",
                    crate::conversions::convert_word_to_int(&pc_word),
                    instr
                );
            }

            // TODO: actually have a trap instruction for shutdown
            if self.memory.o_bus1().read_word()
                == crate::types::Word::state([0, 0, 0, 0, 0, 0, 0, 0, 0])
            {
                warn!("End of program, shutdown..");
                break;
            }

            self.control.update();
            self.pc_inc.update();

            // read register file
            self.primary_reg_mux.update();
            self.reg_dst_mux.update();
            self.register.update();

            self.immediate_extend.update();
            self.alu_src_mux.update();

            self.alu.update();
            self.detect_zero.update();

            self.memory.update();
            self.write_data_mux.update();
            self.register.write_data();

            self.branch_shift.update();
            self.branch_inc.update();
            self.branch_cond_gate.update();
            self.branch_select_mux.update();

            if std::env::var("DEBUG_DEMO").is_ok() {
                println!("{}", self.dump_registers());
            }
        }
    }

    pub fn load_program(&mut self, program: Vec<crate::types::Word>) {
        for (idx, word) in program.iter().enumerate() {
            self.memory.init_data(word, idx * crate::types::WORD_SIZE);
        }
    }

    pub fn dump_registers(&self) -> String {
        self.register.dump_registers()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//
// }
