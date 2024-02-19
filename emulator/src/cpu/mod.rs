use self::instructions::{OPCODE, OPCODES};

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
    // for communication with other components
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

    pub fn step(&mut self) -> u32 {
        // record cycle before executing instruction
        let start_cycle = self.cycles;

        // handle dma
        self.handle_dma();

        // handle early return if cpu is stalled
        if self.stall > 0 {
            self.stall -= 1;
            return 1;
        }

        // handle interrupt
        self.handle_interrupt();

        // fetch instruction
        let opcode = self.bus.read(self.pc) as usize;

        // decode instruction
        let opcode = &OPCODES[opcode];
        let OPCODE {
            instruction,
            mode,
            size,
            cycles,
            extra_cycles,
        } = opcode;

        // update pc to next instruction
        self.pc += size;

        // fetch address of operand and check if page crossed
        // fetching from memory takes extra cycles if page boundary is crossed
        let (address, page_crossed) = mode.fetch_operand_address(self);

        // execute instruction and pass address of operand and addressing mode
        instruction.execute(self, address, mode);

        // update cycles after executing instruction
        if page_crossed {
            self.cycles += cycles + extra_cycles;
        } else {
            self.cycles += cycles;
        }

        // return cycles taken to execute instruction
        self.cycles - start_cycle
    }

    fn handle_dma(&self) {
        todo!()
    }

    fn handle_interrupt(&mut self) {
        todo!()
    }

}
