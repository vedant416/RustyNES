use self::instructions::{OPCODE, OPCODES};
use super::bus::BUS;
mod instructions;

pub enum Interrupt {
    NMI,
    IRQ,
    None,
}

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
    pub bus: BUS,
}

impl CPU {
    pub fn new_cpu(bus: BUS) -> CPU {
        let mut cpu = CPU {
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
        };

        // inital state of cpu
        cpu.sp = 0xFD;
        cpu.pc = cpu.read_16(0xFFFC);
        cpu.cycles = 7;
        cpu.i = true;
        cpu.u = true;
        cpu
    }

    pub fn step(&mut self) -> u32 {
        // record cycle before executing instruction
        let start_cycles = self.cycles;

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

        // fetch address of operand and check if page crossed
        // fetching from memory takes extra cycles if page boundary is crossed
        let (address, page_crossed) = mode.fetch_operand_address(self);

        // update pc to next instruction
        self.pc += size;

        // execute instruction and pass address of operand and addressing mode
        instruction.execute(self, address, mode);

        // update cycles after executing instruction
        if page_crossed {
            self.cycles += cycles + extra_cycles;
        } else {
            self.cycles += cycles;
        }

        // return cycles taken to execute instruction
        self.cycles - start_cycles
    }

    fn handle_dma(&mut self) {
        if self.bus.ppu.dma_triggered() {
            self.stall += 513 + (self.cycles & 1);
        }
    }

    fn handle_interrupt(&mut self) {
        match self.bus.ppu.interrupt_triggered() {
            Interrupt::NMI => self.interrupt(0xFFFA),
            Interrupt::IRQ => self.interrupt(0xFFFE),
            Interrupt::None => {}
        }
    }

    fn interrupt(&mut self, vector: u16) {
        self.push_16(self.pc); // push program counter
        self.php(); // push status register
        self.i = true; // set interrupt disable flag to true
        self.pc = self.read_16(vector); // set program counter to interrupt vector
        self.cycles += 7;
    }
}

impl CPU {
    pub fn get_state(&self) -> CpuState {
        CpuState {
            a: self.a,
            x: self.x,
            y: self.y,
            sp: self.sp,
            pc: self.pc,
            c: self.c,
            z: self.z,
            i: self.i,
            d: self.d,
            b: self.b,
            u: self.u,
            v: self.v,
            n: self.n,
            cycles: self.cycles,
            stall: self.stall,
        }
    }

    pub fn set_state(bus: BUS, state: CpuState) -> CPU {
        CPU {
            a: state.a,
            x: state.x,
            y: state.y,
            sp: state.sp,
            pc: state.pc,
            c: state.c,
            z: state.z,
            i: state.i,
            d: state.d,
            b: state.b,
            u: state.u,
            v: state.v,
            n: state.n,
            cycles: state.cycles,
            stall: state.stall,
            bus,
        }
    }
}

#[derive(Clone)]
pub struct CpuState {
    pub a: u8,  // Accumulator
    pub x: u8,  // X register
    pub y: u8,  // Y register
    pub sp: u8, // Stack pointer

    pub pc: u16, // Program counter

    // Status register flags
    pub c: bool, // Carry (bit 0)
    pub z: bool, // Zero (bit 1)
    pub i: bool, // Interrupt disable (bit 2)
    pub d: bool, // Decimal mode (unsupported on NES, bit 3)
    pub b: bool, // Software interrupt (BRK) (bit 4)
    pub u: bool, // Unused flag (bit 5)
    pub v: bool, // Overflow (bit 6)
    pub n: bool, // Negative (bit 7)

    // state
    pub cycles: u32,
    pub stall: u32,
}
