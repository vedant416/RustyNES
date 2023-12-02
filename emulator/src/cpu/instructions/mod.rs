#[allow(dead_code)]
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
    #[allow(dead_code)]
    pub fn execute(&self) {
        match *self {
            _ => println!("Not implemented yet"),
        }
    }
}
