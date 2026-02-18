use riscv_assembler::assembler::{assemble, AssemblyOutput};
use riscv_decode::decode;
use riscv_decode::types::*;
use riscv_decode::DecodingError;
use riscv_decode::Instruction;
use rstest::rstest;

use nomos::fns::Exec;

#[macro_export]
macro_rules! assemble_and_exec {
    ($code:expr, $regs:expr) => {{
        let inst = $code;
        let inst = assemble($code).unwrap().code[0];
        let decoded = decode(inst).unwrap();
        decoded.exec($regs).unwrap();
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest]
    #[case("add x3, x1, x2", 3, 10, 20, 30)]
    #[case("add x4, x1, x2", 4, 0xFFFFFFFF, 1, 0)]
    #[case("add x5, x1, x2", 5, 0x7FFFFFFF, 1, 0x80000000)]
    #[case("add x6, x1, x2", 6, 0x80000000, 0xFFFFFFFF, 0x7FFFFFFF)]
    #[case("sub x7, x1, x2", 7, 20, 10, 10)]
    #[case("sub x8, x1, x2", 8, 0, 1, 0xFFFFFFFF)]
    #[case("sub x9, x1, x2", 9, 0x80000000, 1, 0x7FFFFFFF)]
    #[case("sub x10, x1, x2", 10, 0x7FFFFFFF, 0xFFFFFFFF, 0x80000000)]
    #[case("and x8, x1, x2", 8, 100, 60, 36)]
    #[case("or x3, x1, x2", 3, 100, 60, 124)]
    #[case("or x4, x1, x2", 4, 0xFFFFFFFF, 0, 0xFFFFFFFF)]
    #[case("xor x10, x1, x2", 10, 100, 60, 88)]
    #[case("xor x11, x1, x2", 11, 0xFFFFFFFF, 0, 0xFFFFFFFF)]
    #[case("xor x12, x1, x2", 12, 0xFFFFFFFF, 0xFFFFFFFF, 0)]
    #[case("sll x15, x1, x2", 15, 1, 2, 4)]
    #[case("srl x2, x1, x2", 2, 4, 2, 1)]
    #[case("sra x7, x1, x2", 7, 0x80000000, 1, 0xC0000000)]
    #[case("sra x8, x1, x2", 8, 0x7FFFFFFF, 1, 0x3FFFFFFF)]
    #[case("slt x16, x1, x2", 16, 10, 20, 1)]
    #[case("slt x31, x1, x2", 31, 20, 10, 0)]
    #[case("sltu x11, x1, x2", 11, 20, 20, 0)]
    #[case("sltu x12, x1, x2", 12, 10, 20, 1)]
    #[case("sltu x7, x1, x2", 7, 20, 10, 0)]
    fn test_exec_rtype(
        #[case] code: &str,
        #[case] rd: usize,
        #[case] rs1: u32,
        #[case] rs2: u32,
        #[case] expected: u32,
    ) {
        let mut regs: [u32; 32] = [0; 32];
        regs[1] = rs1;
        regs[2] = rs2;
        assemble_and_exec!(code, &mut regs);
        assert_eq!(regs[rd], expected);
    }

    #[rstest]
    #[case("addi x3, x2, 42", 3, 2, 10, 42, 52)]
    #[case("addi x4, x3, -1", 4, 3, 0, 0xFFFFFFFF, 0xFFFFFFFF)]
    #[case("addi x5, x4, -1", 5, 4, 0, 0xFFFFFFFF, 0xFFFFFFFF)]
    #[case("addi x6, x12, 1", 6, 12, 0xFFFFFFFF, 1, 0)]
    #[case("addi x7, x13, -50", 7, 13, 100, -50i32 as u32, 50)]
    #[case("addi x8, x14, 0", 8, 14, 12345, 0, 12345)]
    #[case("ori x9, x15, 0xFF", 9, 15, 0x12345678, 0xFF, 0x123456FF)]
    #[case("slti x15, x21, 100", 15, 21, 50, 100, 1)]
    #[case("slti x16, x22, 100", 16, 22, 150, 100, 0)]
    #[case("sltiu x17, x23, 100", 17, 23, 50, 100, 1)]
    #[case("sltiu x18, x24, 100", 18, 24, 150, 100, 0)]
    #[case("sltiu x19, x25, 100", 19, 25, 0xFFFFFFFF, 100, 0)]
    fn test_exec_itype(
        #[case] code: &str,
        #[case] rd: usize,
        #[case] rs1: usize,
        #[case] rs1_val: u32,
        #[case] imm: u32,
        #[case] expected: u32,
    ) {
        let mut regs: [u32; 32] = [0; 32];
        regs[rs1] = rs1_val;
        assemble_and_exec!(code, &mut regs);
        assert_eq!(regs[rd], expected);
    }
}
