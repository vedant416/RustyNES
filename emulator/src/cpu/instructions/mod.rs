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
}
