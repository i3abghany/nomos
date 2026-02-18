use riscv_decode::decode;

use crate::exec::Exec;

const MEM_SIZE: usize = 1024 * 1024;
pub struct Cpu {
    pub regs: [u32; 32],
    pub mem: Vec<u8>,
    pub pc: u32,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            regs: [0; 32],
            mem: vec![0; MEM_SIZE],
            pc: 0,
        }
    }

    pub fn load_program(&mut self, program: Vec<u32>) {
        self.mem[..program.len() * 4].copy_from_slice(unsafe {
            std::slice::from_raw_parts(program.as_ptr() as *const u8, program.len() * 4)
        });
    }

    pub fn step(&mut self) {
        let inst = u32::from_le_bytes([
            self.mem[self.pc as usize],
            self.mem[self.pc as usize + 1],
            self.mem[self.pc as usize + 2],
            self.mem[self.pc as usize + 3],
        ]);

        let decoded = decode(inst).unwrap();
        decoded.exec(&mut self.regs).unwrap();
        self.pc += 4;
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
