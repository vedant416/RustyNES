use crate::{
    buffer::{self, Buffer},
    rom::{create_cartridge, ROM},
};

use self::instructions::{OPCODE, OPCODES};
use super::bus::BUS;
mod instructions;

pub enum Interrupt {
    NMI,
    IRQ,
    None,
}

#[derive(Default)]
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

    // buffer
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
        if self.bus.ppu.nmi_triggered() {
            self.interrupt(0xFFFA)
        } else {
            if !self.i {
                let cartridge_irq = self.bus.ppu.cartridge.irq_triggered();
                let apu_irq = self.bus.apu.irq_triggered();
                if cartridge_irq || apu_irq {
                    self.interrupt(0xFFFE);
                }
            }
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

// Save and Load /////////////////////
impl CPU {
    fn encode_cpu(&self, buffer: &mut buffer::Buffer) {
        buffer.write_u8(self.a);
        buffer.write_u8(self.x);
        buffer.write_u8(self.y);
        buffer.write_u8(self.sp);
        buffer.write_u16(self.pc);

        buffer.write_bool(self.c);
        buffer.write_bool(self.z);
        buffer.write_bool(self.i);
        buffer.write_bool(self.d);
        buffer.write_bool(self.b);
        buffer.write_bool(self.u);
        buffer.write_bool(self.v);
        buffer.write_bool(self.n);

        buffer.write_u32(self.cycles);
        buffer.write_u32(self.stall);
    }

    fn decode_cpu(&mut self, buffer: &mut buffer::Buffer) {
        self.a = buffer.read_u8();
        self.x = buffer.read_u8();
        self.y = buffer.read_u8();
        self.sp = buffer.read_u8();
        self.pc = buffer.read_u16();

        self.c = buffer.read_bool();
        self.z = buffer.read_bool();
        self.i = buffer.read_bool();
        self.d = buffer.read_bool();
        self.b = buffer.read_bool();
        self.u = buffer.read_bool();
        self.v = buffer.read_bool();
        self.n = buffer.read_bool();

        self.cycles = buffer.read_u32();
        self.stall = buffer.read_u32();
    }

    pub fn encode(&mut self, buffer: &mut buffer::Buffer) {
        *buffer = Buffer::new_buffer();
        self.bus.ppu.cartridge.data().encode(buffer);
        self.bus.ppu.cartridge.encode(buffer);
        self.bus.ppu.encode(buffer);
        self.bus.controller.encode(buffer);
        self.bus.encode(buffer);
        self.encode_cpu(buffer);
    }

    pub fn decode(&mut self, buffer: &mut buffer::Buffer) {
        let rom = ROM::decode(buffer);
        self.bus.ppu.cartridge = create_cartridge(rom.mapper_id, rom);
        self.bus.ppu.cartridge.decode(buffer);
        self.bus.ppu.decode(buffer);
        self.bus.controller.decode(buffer);
        self.bus.decode(buffer);
        self.decode_cpu(buffer);
        // reset read index after decoding
        buffer.index = 0;
    }
}
