use crate::components::platform::bus::Bus;
use crate::components::platform::decoder::Decoder;
use crate::components::platform::mux::Mux;
use crate::components::platform::wire::{Wire, read};
use crate::components::registers::register::Register;
use crate::components::{BinaryBusOutputComponent, Component, UnaryBusOutputComponent};
use crate::types::{Trit, WORD_SIZE, Word};

pub struct RegisterFile {
    registers: [Register; WORD_SIZE],

    reg1_select: [Wire; 2],
    reg2_select: [Wire; 2],

    reg_write_select: [Wire; 2],
    write_enable: Wire,
    write_data: Bus,
    // decode the reg_write_select signals to determine which register to write to
    write_decoder: Decoder,

    read_data1: Bus,
    read_data2: Bus,
    // muxes to select which register to read for data1 and data2 lines
    data1_mux: Mux,
    data2_mux: Mux,
}

impl RegisterFile {
    pub fn new(
        reg1_select: [Wire; 2],
        reg2_select: [Wire; 2],
        reg_write_select: [Wire; 2],
        write_enable: Wire,
        write_data: Bus,
    ) -> Self {
        let write_decoder = Decoder::new(Vec::from(reg_write_select.clone()), write_enable.clone());
        let registers = std::array::from_fn(|i| {
            Register::new(
                format!("reg{i}"),
                write_data.clone(),
                write_decoder.get_output(i).clone(),
            )
        });
        let data1_mux = Mux::new(
            Vec::from(reg1_select.clone()),
            registers.iter().map(Register::o_bus1).cloned().collect(),
        );
        let data2_mux = Mux::new(
            Vec::from(reg2_select.clone()),
            registers.iter().map(Register::o_bus1).cloned().collect(),
        );
        RegisterFile {
            registers,

            reg1_select,
            reg2_select,

            reg_write_select,
            write_enable,
            write_data,
            write_decoder,

            read_data1: data1_mux.o_bus1().clone(),
            read_data2: data2_mux.o_bus1().clone(),
            data1_mux,
            data2_mux,
        }
    }
}

impl Component for RegisterFile {
    fn update(&mut self) {
        // update write inputs first
        self.write_decoder.update();

        self.registers.iter_mut().for_each(Register::update);

        // then update read muxes to reflect new register values
        self.data1_mux.update();
        self.data2_mux.update();
    }
}

impl BinaryBusOutputComponent for RegisterFile {
    /// data1 output
    fn o_bus1(&self) -> &Bus {
        &self.read_data1
    }
    /// data2 output
    fn o_bus2(&self) -> &Bus {
        &self.read_data2
    }
}

#[cfg(test)]
mod tests {
    use crate::components::platform::bus::Bus;
    use crate::components::platform::wire::{wire, write};
    use crate::components::registers::register_file::RegisterFile;
    use crate::components::{BinaryBusOutputComponent, Component};
    use crate::conversions;
    use crate::conversions::convert_int_to_word;
    use crate::types::{Trit, WORD_SIZE, Word};

    #[test]
    fn test_register_file() {
        let reg1_select = [wire(Trit::N), wire(Trit::N)];
        let reg2_select = [wire(Trit::N), wire(Trit::N)];
        let reg_write_select = [wire(Trit::N), wire(Trit::N)];
        let write_enable = wire(Trit::N);
        let write_data = Bus::new();

        let mut reg_file = RegisterFile::new(
            reg1_select.clone(),
            reg2_select.clone(),
            reg_write_select.clone(),
            write_enable.clone(),
            write_data.clone(),
        );

        // Write to register 0
        write(&reg_write_select[0], &Trit::N);
        write(&reg_write_select[1], &Trit::N);
        write(&write_enable, &Trit::P);
        write_data.write_word(&Word::state([1, 0, 0, 0, 0, 0, 0, 0, 0]));
        reg_file.update();

        // Read from register 0
        write(&reg1_select[0], &Trit::N);
        write(&reg1_select[1], &Trit::N);
        write(&write_enable, &Trit::Z);
        reg_file.update();
        assert_eq!(
            reg_file.o_bus1().read_word(),
            Word::state([1, 0, 0, 0, 0, 0, 0, 0, 0])
        );

        // Read from register 1 (should be zero)
        write(&reg2_select[0], &Trit::Z);
        write(&reg2_select[1], &Trit::N);
        reg_file.update();
        assert_eq!(reg_file.o_bus2().read_word(), Word::state([0; WORD_SIZE]));
    }

    #[test]
    fn test_read_from_multiple_registers() {
        let reg1_select = [wire(Trit::N), wire(Trit::N)];
        let reg2_select = [wire(Trit::N), wire(Trit::N)];
        let reg_write_select = [wire(Trit::N), wire(Trit::N)];
        let write_control = wire(Trit::N);
        let write_data = Bus::new();

        let mut reg_file = RegisterFile::new(
            reg1_select.clone(),
            reg2_select.clone(),
            reg_write_select.clone(),
            write_control.clone(),
            write_data.clone(),
        );

        for i in 0..WORD_SIZE {
            write(&write_control, &Trit::P);

            write_data.write_word(&convert_int_to_word(i as i32));

            let select_word = conversions::convert_int_to_unsigned_word(i as i32);
            write(&reg_write_select[0], select_word.get_trit(0));
            write(&reg_write_select[1], select_word.get_trit(1));
            reg_file.update();
        }

        write(&write_control, &Trit::Z); // disable writing
        for i in 0..WORD_SIZE {
            for j in 0..WORD_SIZE {
                let select_word1 = conversions::convert_int_to_unsigned_word(i as i32);
                let select_word2 = conversions::convert_int_to_unsigned_word(j as i32);
                write(&reg1_select[0], select_word1.get_trit(0));
                write(&reg1_select[1], select_word1.get_trit(1));
                write(&reg2_select[0], select_word2.get_trit(0));
                write(&reg2_select[1], select_word2.get_trit(1));
                reg_file.update();

                assert_eq!(
                    reg_file.o_bus1().read_word(),
                    convert_int_to_word(i as i32),
                    "Failed reading register {i}"
                );
                assert_eq!(
                    reg_file.o_bus2().read_word(),
                    convert_int_to_word(j as i32),
                    "Failed reading register {j}"
                );
            }
        }
    }

    #[test]
    fn test_reset_register() {
        let reg1_select = [wire(Trit::N), wire(Trit::N)];
        let reg2_select = [wire(Trit::N), wire(Trit::N)];
        let reg_write_select = [wire(Trit::N), wire(Trit::N)];
        let write_control = wire(Trit::N);
        let write_data = Bus::new();
        let written_word = Word::state([1, 0, 0, 0, 0, 0, 0, 0, 0]);

        let mut reg_file = RegisterFile::new(
            reg1_select.clone(),
            reg2_select.clone(),
            reg_write_select.clone(),
            write_control.clone(),
            write_data.clone(),
        );

        // Write to register 0
        write(&reg_write_select[0], &Trit::N);
        write(&reg_write_select[1], &Trit::N);
        write(&write_control, &Trit::P);
        write_data.write_word(&written_word);
        reg_file.update();

        // Read from register 0 to confirm write
        write(&reg1_select[0], &Trit::N);
        write(&reg1_select[1], &Trit::N);
        write(&write_control, &Trit::Z);
        reg_file.update();
        assert_eq!(
            reg_file.o_bus1().read_word(),
            written_word,
            "Failed to write register 0"
        );

        // Reset register 0
        write(&write_control, &Trit::N);
        write(&reg_write_select[0], &Trit::N);
        write(&reg_write_select[1], &Trit::N);
        reg_file.update();

        // Read from register 0 to confirm reset
        write(&reg1_select[0], &Trit::N);
        write(&reg1_select[1], &Trit::N);
        write(&write_control, &Trit::Z);
        reg_file.update();
        assert_eq!(
            reg_file.o_bus1().read_word(),
            Word::state([0; WORD_SIZE]),
            "Failed to reset register 0"
        );
    }
}
