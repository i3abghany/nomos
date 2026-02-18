use riscv_decode::{decode, instruction_length};

use crate::exec::Exec;

const MEM_SIZE: usize = 2 * 1024 * 1024;
pub struct Hart {
    pub regs: [u32; 32],
    pub mem: Vec<u8>,
    pub pc: u32,
}

impl Hart {
    pub fn new() -> Self {
        Hart {
            regs: [0; 32],
            mem: vec![0; MEM_SIZE],
            pc: 0,
        }
    }

    pub fn load_program(&mut self, program: Vec<u8>) {
        self.mem[..program.len()].copy_from_slice(&program);
    }

    pub fn step(&mut self) {
        let inst_bytes = &self.mem[self.pc as usize..self.pc as usize + 4];
        let low_16_bit = u16::from_le_bytes([inst_bytes[0], inst_bytes[1]]);
        let insn_length = instruction_length(low_16_bit);

        let decoded = if insn_length == 2 {
            println!("Compressed instruction: 0x{:04x}", low_16_bit);
            decode(low_16_bit as u32).unwrap()
        } else {
            let inst =
                u32::from_le_bytes([inst_bytes[0], inst_bytes[1], inst_bytes[2], inst_bytes[3]]);
            decode(inst).unwrap()
        };

        decoded.exec(self).unwrap();
        self.pc += insn_length as u32;
    }

    fn abi_name(reg: usize) -> &'static str {
        match reg {
            0 => "zero",
            1 => "ra",
            2 => "sp",
            3 => "gp",
            4 => "tp",
            5 => "t0",
            6 => "t1",
            7 => "t2",
            8 => "s0",
            9 => "s1",
            10 => "a0",
            11 => "a1",
            12 => "a2",
            13 => "a3",
            14 => "a4",
            15 => "a5",
            16 => "a6",
            17 => "a7",
            18 => "s2",
            19 => "s3",
            20 => "s4",
            21 => "s5",
            22 => "s6",
            23 => "s7",
            24 => "s8",
            25 => "s9",
            26 => "s10",
            27 => "s11",
            28 => "t3",
            29 => "t4",
            30 => "t5",
            31 => "t6",
            _ => unreachable!(),
        }
    }

    pub fn log_state(&self) {
        println!("PC: {:#010x}", self.pc);
        for i in 0..32 {
            print!("{: <4}: {:#010x} ", Self::abi_name(i), self.regs[i]);
            if (i + 1) % 4 == 0 {
                println!();
            } else {
                print!("- ");
            }
        }
    }
}
