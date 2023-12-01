mod instructions;
#[allow(dead_code)]  // todo: remove this
pub struct CPU {
    a: u8, // Accumulator
    x: u8, // X register
    y: u8, // Y register 
    sp: u8, // Stack pointer
    
    pc: u16, // Program counter

    // Status register flags
    c: bool, // Carry (bit 0)
    z: bool, // Zero (bit 1)
    i: bool, // Interrupt disable (bit 2)
    d: bool, // Decimal mode (unsupported on NES, bit 3)
    b: bool, // Software interrupt (BRK) (bit 4)
    u: bool, // Unused flag (bit 5)
    v: bool, // Overflow (bit 6)
    n: bool, // Negative (bit 7)
}

impl CPU {
    pub fn new_cpu() -> CPU {
        CPU {
            a: 0,
            x: 0,
            y: 0,
            sp: 0,
            
            pc: 0,
            
            c: false,
            z: false,
            i: false,
            d: false,
            b: false,
            u: false,
            v: false,
            n: false,
        }
    }
}
