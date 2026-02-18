use riscv_decode::DecodingError;
use riscv_decode::Instruction;

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
    fn exec(&self, regs: &mut [u32; 32]) -> Result<(), DecodingError>;
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
    fn exec(&self, regs: &mut [u32; 32]) -> Result<(), DecodingError> {
        match self {
            Instruction::Add(inst) => exec_add(rd!(inst), rs1!(inst), rs2!(inst), regs),
            Instruction::Sub(inst) => exec_sub(rd!(inst), rs1!(inst), rs2!(inst), regs),
            Instruction::And(inst) => exec_and(rd!(inst), rs1!(inst), rs2!(inst), regs),
            Instruction::Or(inst) => exec_or(rd!(inst), rs1!(inst), rs2!(inst), regs),
            Instruction::Xor(inst) => exec_xor(rd!(inst), rs1!(inst), rs2!(inst), regs),
            Instruction::Sll(inst) => exec_sll(rd!(inst), rs1!(inst), rs2!(inst), regs),
            Instruction::Srl(inst) => exec_srl(rd!(inst), rs1!(inst), rs2!(inst), regs),
            Instruction::Sra(inst) => exec_sra(rd!(inst), rs1!(inst), rs2!(inst), regs),
            Instruction::Slt(inst) => exec_slt(rd!(inst), rs1!(inst), rs2!(inst), regs),
            Instruction::Sltu(inst) => exec_sltu(rd!(inst), rs1!(inst), rs2!(inst), regs),
            Instruction::Lui(inst) => exec_lui(rd!(inst), inst.imm(), regs),
            Instruction::Addi(inst) => exec_addi(rd!(inst), rs1!(inst), imm!(inst), regs),
            Instruction::Slti(inst) => exec_slti(rd!(inst), rs1!(inst), imm!(inst), regs),
            Instruction::Sltiu(inst) => exec_sltiu(rd!(inst), rs1!(inst), imm!(inst), regs),
            Instruction::Xori(inst) => exec_xori(rd!(inst), rs1!(inst), imm!(inst), regs),
            Instruction::Ori(inst) => exec_ori(rd!(inst), rs1!(inst), imm!(inst), regs),
            Instruction::Andi(inst) => exec_andi(rd!(inst), rs1!(inst), imm!(inst), regs),
            Instruction::Slli(inst) => exec_slli(rd!(inst), rs1!(inst), inst.shamt(), regs),
            Instruction::Srli(inst) => exec_srli(rd!(inst), rs1!(inst), inst.shamt(), regs),
            Instruction::Srai(inst) => exec_srai(rd!(inst), rs1!(inst), inst.shamt(), regs),
            _ => unreachable!(),
        }

        return Ok(());
    }
}
