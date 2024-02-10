use super::bus::BUS;

mod instructions;
pub struct CPU {
    a: u8,  // Accumulator
    x: u8,  // X register
    y: u8,  // Y register
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

    // state
    cycles: u32,
    stall: u32,

    bus: BUS,
}

impl CPU {
    pub fn new_cpu(bus: BUS) -> CPU {
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

            cycles: 0,
            stall: 0,

            bus,
        }
    }

    pub fn step(&mut self) -> usize {
        todo!()
        // handle early return due to dma
        // handle interrupts
        // record cycle before executing instruction
        // fetch instruction
        // decode instruction
        // fetch address of operand and addressing mode
        // update pc and cycles
        // execute instruction(address, addressing mode)
        // return cycles taken (current cycle - recorded cycle)
    }
}
