use riscv_decode::DecodingError;
use riscv_decode::Instruction;

use crate::hart::Hart;

fn exec_add(rd: usize, rs1: usize, rs2: usize, regs: &mut [u32; 32]) {
    regs[rd] = regs[rs1].wrapping_add(regs[rs2]);
}

fn exec_addi(rd: usize, rs1: usize, imm: u32, regs: &mut [u32; 32]) {
    regs[rd] = regs[rs1].wrapping_add(imm);
}

fn exec_lui(rd: usize, imm: u32, regs: &mut [u32; 32]) {
    regs[rd] = imm;
}

fn exec_sub(rd: usize, rs1: usize, rs2: usize, regs: &mut [u32; 32]) {
    regs[rd] = regs[rs1].wrapping_sub(regs[rs2]);
}

fn exec_and(rd: usize, rs1: usize, rs2: usize, regs: &mut [u32; 32]) {
    regs[rd] = regs[rs1] & regs[rs2];
}

fn exec_or(rd: usize, rs1: usize, rs2: usize, regs: &mut [u32; 32]) {
    regs[rd] = regs[rs1] | regs[rs2];
}

fn exec_xor(rd: usize, rs1: usize, rs2: usize, regs: &mut [u32; 32]) {
    regs[rd] = regs[rs1] ^ regs[rs2];
}

fn exec_sll(rd: usize, rs1: usize, rs2: usize, regs: &mut [u32; 32]) {
    let shamt = regs[rs2] & 0x1F;
    regs[rd] = regs[rs1] << shamt;
}

fn exec_srl(rd: usize, rs1: usize, rs2: usize, regs: &mut [u32; 32]) {
    let shamt = regs[rs2] & 0x1F;
    regs[rd] = regs[rs1] >> shamt;
}

fn exec_sra(rd: usize, rs1: usize, rs2: usize, regs: &mut [u32; 32]) {
    let shamt = regs[rs2] & 0x1F;
    regs[rd] = ((regs[rs1] as i32) >> shamt) as u32;
}

fn exec_slt(rd: usize, rs1: usize, rs2: usize, regs: &mut [u32; 32]) {
    regs[rd] = if (regs[rs1] as i32) < (regs[rs2] as i32) {
        1
    } else {
        0
    };
}

fn exec_sltu(rd: usize, rs1: usize, rs2: usize, regs: &mut [u32; 32]) {
    regs[rd] = if regs[rs1] < regs[rs2] { 1 } else { 0 };
}

fn exec_slti(rd: usize, rs1: usize, imm: u32, regs: &mut [u32; 32]) {
    regs[rd] = if (regs[rs1] as i32) < (imm as i32) {
        1
    } else {
        0
    };
}

fn exec_sltiu(rd: usize, rs1: usize, imm: u32, regs: &mut [u32; 32]) {
    regs[rd] = if regs[rs1] < imm { 1 } else { 0 };
}

fn exec_xori(rd: usize, rs1: usize, imm: u32, regs: &mut [u32; 32]) {
    regs[rd] = regs[rs1] ^ imm;
}

fn exec_ori(rd: usize, rs1: usize, imm: u32, regs: &mut [u32; 32]) {
    regs[rd] = regs[rs1] | imm;
}

fn exec_andi(rd: usize, rs1: usize, imm: u32, regs: &mut [u32; 32]) {
    regs[rd] = regs[rs1] & imm;
}

fn exec_slli(rd: usize, rs1: usize, shamt: u32, regs: &mut [u32; 32]) {
    let shamt = shamt & 0x1F;
    regs[rd] = regs[rs1] << shamt;
}

fn exec_srli(rd: usize, rs1: usize, shamt: u32, regs: &mut [u32; 32]) {
    let shamt = shamt & 0x1F;
    regs[rd] = regs[rs1] >> shamt;
}

fn exec_srai(rd: usize, rs1: usize, shamt: u32, regs: &mut [u32; 32]) {
    let shamt = shamt & 0x1F;
    regs[rd] = ((regs[rs1] as i32) >> shamt) as u32;
}

pub trait Exec {
    fn exec(&self, regs: &mut Hart) -> Result<(), DecodingError>;
}

macro_rules! rd {
    ($i:ident) => {
        $i.rd() as usize
    };
}
macro_rules! rs1 {
    ($i:ident) => {
        $i.rs1() as usize
    };
}
macro_rules! rs2 {
    ($i:ident) => {
        $i.rs2() as usize
    };
}
macro_rules! imm {
    ($i:ident) => {{
        let imm = $i.imm();
        ((imm as i32) << 20 >> 20) as u32
    }};
}

impl Exec for Instruction {
    fn exec(&self, r: &mut Hart) -> Result<(), DecodingError> {
        let r = &mut r.regs;
        match self {
            Instruction::Add(i) => exec_add(rd!(i), rs1!(i), rs2!(i), r),
            Instruction::Sub(i) => exec_sub(rd!(i), rs1!(i), rs2!(i), r),
            Instruction::And(i) => exec_and(rd!(i), rs1!(i), rs2!(i), r),
            Instruction::Or(i) => exec_or(rd!(i), rs1!(i), rs2!(i), r),
            Instruction::Xor(i) => exec_xor(rd!(i), rs1!(i), rs2!(i), r),
            Instruction::Sll(i) => exec_sll(rd!(i), rs1!(i), rs2!(i), r),
            Instruction::Srl(i) => exec_srl(rd!(i), rs1!(i), rs2!(i), r),
            Instruction::Sra(i) => exec_sra(rd!(i), rs1!(i), rs2!(i), r),
            Instruction::Slt(i) => exec_slt(rd!(i), rs1!(i), rs2!(i), r),
            Instruction::Sltu(i) => exec_sltu(rd!(i), rs1!(i), rs2!(i), r),
            Instruction::Lui(i) => exec_lui(rd!(i), imm!(i), r),
            Instruction::Addi(i) => exec_addi(rd!(i), rs1!(i), imm!(i), r),
            Instruction::Slti(i) => exec_slti(rd!(i), rs1!(i), imm!(i), r),
            Instruction::Sltiu(i) => exec_sltiu(rd!(i), rs1!(i), imm!(i), r),
            Instruction::Xori(i) => exec_xori(rd!(i), rs1!(i), imm!(i), r),
            Instruction::Ori(i) => exec_ori(rd!(i), rs1!(i), imm!(i), r),
            Instruction::Andi(i) => exec_andi(rd!(i), rs1!(i), imm!(i), r),
            Instruction::Slli(i) => exec_slli(rd!(i), rs1!(i), i.shamt(), r),
            Instruction::Srli(i) => exec_srli(rd!(i), rs1!(i), i.shamt(), r),
            Instruction::Srai(i) => exec_srai(rd!(i), rs1!(i), i.shamt(), r),
            _ => unimplemented!("Instruction {:?} not implemented", self),
        }

        return Ok(());
    }
}
