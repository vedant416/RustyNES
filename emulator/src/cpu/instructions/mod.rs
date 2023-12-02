use super::CPU;
mod flag_instructions;
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
    pub fn execute(&self, cpu: &mut CPU) {
        match *self {
            //// BITWISE ////
            
            //// BRANCH ////
            
            //// COMPARE ////
            
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
            
            //// REGISTER //// 
            
            //// STACK ////
            
            //// OTHER ////
            
            //// ILLEGAL ////
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
