use super::CPU;

mod branch_instructions;
mod compare_instructions;
mod flag_instructions;
mod jump_instructions;
mod memory_instructions;
mod register_instructions;

enum Instruction {
    //// BITWISE ////
    AND,
    EOR,
    ORA,
    ASL,
    LSR,
    ROL,
    ROR,

    //// BRANCH ////
    BPL,
    BMI,
    BVC,
    BVS,
    BCC,
    BCS,
    BNE,
    BEQ,

    //// COMPARE ////
    CMP,
    CPX,
    CPY,
    BIT,

    //// FLAG ////
    CLC,
    CLD,
    CLI,
    CLV,
    SEC,
    SED,
    SEI,

    //// JUMP ////
    JMP,
    JSR,
    RTI,
    RTS,

    //// MATH ////
    ADC,
    SBC,

    //// MEMORY ////
    LDA,
    LDX,
    LDY,
    STA,
    STX,
    STY,
    INC,
    DEC,

    //// REGISTER ////
    TAX,
    TAY,
    TXA,
    TYA,
    INX,
    INY,
    DEX,
    DEY,
    TXS,
    TSX,

    //// STACK ////
    PHA,
    PHP,
    PLA,
    PLP,

    //// OTHER ////
    BRK,
    NOP,

    //// ILLEGAL ////
    AHX,
    ALR,
    ANC,
    ARR,
    AXS,
    DCP,
    ISB,
    JAM,
    LAS,
    LAX,
    RLA,
    RRA,
    SAX,
    SHX,
    SHY,
    SLO,
    SRE,
    TAS,
    XAA,
}

impl Instruction {
    pub fn execute(&self, cpu: &mut CPU, addr: u16) {
        match *self {
            //// BITWISE ////

            //// BRANCH ////
            Instruction::BPL => cpu.bpl(addr),
            Instruction::BMI => cpu.bmi(addr),
            Instruction::BVC => cpu.bvc(addr),
            Instruction::BVS => cpu.bvs(addr),
            Instruction::BCC => cpu.bcc(addr),
            Instruction::BCS => cpu.bcs(addr),
            Instruction::BNE => cpu.bne(addr),
            Instruction::BEQ => cpu.beq(addr),

            //// COMPARE ////
            Instruction::CMP => cpu.cmp(addr),
            Instruction::CPX => cpu.cpx(addr),
            Instruction::CPY => cpu.cpy(addr),
            Instruction::BIT => cpu.bit(addr),

            //// FLAG ////
            Instruction::CLC => cpu.clc(),
            Instruction::CLD => cpu.cld(),
            Instruction::CLI => cpu.cli(),
            Instruction::CLV => cpu.clv(),
            Instruction::SEC => cpu.sec(),
            Instruction::SED => cpu.sed(),
            Instruction::SEI => cpu.sei(),

            //// JUMP ////

            //// MATH ////

            //// MEMORY ////
            Instruction::LDA => cpu.lda(addr),
            Instruction::LDX => cpu.ldx(addr),
            Instruction::LDY => cpu.ldy(addr),
            Instruction::STA => cpu.sta(addr),
            Instruction::STX => cpu.stx(addr),
            Instruction::STY => cpu.sty(addr),
            Instruction::INC => cpu.inc(addr),
            Instruction::DEC => cpu.dec(addr),

            //// REGISTER ////
            Instruction::TAX => cpu.tax(),
            Instruction::TAY => cpu.tay(),
            Instruction::TXA => cpu.txa(),
            Instruction::TYA => cpu.tya(),
            Instruction::INX => cpu.inx(),
            Instruction::INY => cpu.iny(),
            Instruction::DEX => cpu.dex(),
            Instruction::DEY => cpu.dey(),
            Instruction::TXS => cpu.txs(),
            Instruction::TSX => cpu.tsx(),

            //// STACK ////

            //// OTHER ////

            //// ILLEGAL ////
            Instruction::AHX => (),
            Instruction::ALR => (),
            Instruction::ANC => (),
            Instruction::ARR => (),
            Instruction::AXS => (),
            Instruction::DCP => (),
            Instruction::ISB => (),
            Instruction::JAM => (),
            Instruction::LAS => (),
            Instruction::LAX => (),
            Instruction::RLA => (),
            Instruction::RRA => (),
            Instruction::SAX => (),
            Instruction::SHX => (),
            Instruction::SHY => (),
            Instruction::SLO => (),
            Instruction::SRE => (),
            Instruction::TAS => (),
            Instruction::XAA => (),
            _ => println!("Not implemented yet"),
        }
    }
}

// utils for instructions
impl CPU {
    ///// IO utils //////////////////////////////
    fn read(&mut self, addr: u16) -> u8 {
        todo!("read");
    }

    fn write(&mut self, addr: u16, val: u8) {
        todo!("write");
    }

    ///// Stack utils ///////////////////////////
    fn pull_8(&mut self) -> u8 {
        todo!("pull_8");
    }

    fn pull_16(&mut self) -> u16 {
        todo!("pull_16");
    }

    fn push_8(&mut self, val: u8) {
        todo!("push_8");
    }

    fn push_16(&mut self, val: u16) {
        todo!("push_16");
    }

    ///// Status flag update utils //////////////
    fn get_flags(&self) -> u8 {
        todo!("get_flags");
    }

    fn set_flags(&mut self, flags: u8) {
        todo!("set_flags");
    }

    fn update_zn_flags(&mut self, val: u8) {
        todo!("update_zn_flags");
    }
}
