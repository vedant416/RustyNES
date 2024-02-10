use super::CPU;

//// module declarations ///////////////////////
mod bitwise_instructions;
mod branch_instructions;
mod compare_instructions;
mod flag_instructions;
mod jump_instructions;
mod math_instructions;
mod memory_instructions;
mod register_instructions;
mod stack_instructions;

//// Utils //////////////////
impl CPU {
    //// IO utils ////
    fn read(&mut self, addr: u16) -> u8 {
        todo!("read");
    }

    fn write(&mut self, addr: u16, val: u8) {
        todo!("write");
    }

    fn read_16(&mut self, addr: u16) -> u16 {
        let lo = self.read(addr) as u16;
        let hi = self.read(addr + 1) as u16;
        (hi << 8) | lo
    }

    fn read_16_from_same_page(&mut self, addr: u16) -> u16 {
        let lo = self.read(addr) as u16;
        let hi = if addr & 0xFF != 0xFF {
            self.read(addr + 1) as u16
        } else {
            self.read(addr & 0xFF00) as u16
        };
        (hi << 8) | lo
    }

    //// Stack utils ////
    fn push_8(&mut self, val: u8) {
        self.write(0x100 | self.sp as u16, val);
        self.sp = self.sp.wrapping_sub(1);
    }

    fn pull_8(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        self.read(0x100 | self.sp as u16)
    }

    fn push_16(&mut self, val: u16) {
        let hi = (val >> 8) as u8;
        let lo = (val & 0xFF) as u8;
        self.push_8(hi);
        self.push_8(lo);
    }

    fn pull_16(&mut self) -> u16 {
        let lo = self.pull_8() as u16;
        let hi = self.pull_8() as u16;
        (hi << 8) | lo
    }

    //// Status flag update utils ////
    fn get_flags(&self) -> u8 {
        (self.c as u8)
        | (self.z as u8) << 1
        | (self.i as u8) << 2
        | (self.d as u8) << 3
        | (self.b as u8) << 4
        | (self.u as u8) << 5
        | (self.v as u8) << 6
        | (self.n as u8) << 7
    }

    fn set_flags(&mut self, data: u8) {
        self.c = data & 0x01 != 0;
        self.z = data & 0x02 != 0;
        self.i = data & 0x04 != 0;
        self.d = data & 0x08 != 0;
        self.b = data & 0x10 != 0;
        self.u = data & 0x20 != 0;
        self.v = data & 0x40 != 0;
        self.n = data & 0x80 != 0;
    }

    fn update_zn_flags(&mut self, data: u8) {
        self.n = data & 0x80 != 0;
        self.z = data == 0;
    }
}

//// Addressing Mode //////////////////////////
pub enum AddressingMode {
    Accumulator,
    Implied,

    Immediate,

    Relative,

    ZeroPage,
    ZeroPageX,
    ZeroPageY,

    Absolute,
    AbsoluteX,
    AbsoluteY,

    Indirect,
    IndirectX,
    IndirectY,
}

impl AddressingMode {
    pub fn get_address(&self, cpu: &mut CPU) -> (u16, bool) {
        let pc = cpu.pc;
        match *self {
            AddressingMode::Accumulator => (0, false),

            AddressingMode::Implied => (0, false),

            AddressingMode::Immediate => (pc + 1, false),

            AddressingMode::Relative => {
                let offset = cpu.read(pc + 1) as i8 as i16;
                let pc = pc as i16;
                let addr = (pc + offset + 2) as u16;
                (addr, false)
            }

            //// ZeroPage ////
            AddressingMode::ZeroPage => {
                let addr = cpu.read(pc + 1) as u16;
                (addr, false)
            }

            AddressingMode::ZeroPageX => {
                let addr = cpu.read(pc + 1).wrapping_add(cpu.x) as u16;
                (addr, false)
            }

            AddressingMode::ZeroPageY => {
                let addr = cpu.read(pc + 1).wrapping_add(cpu.y) as u16;
                (addr, false)
            }

            //// Absolute ////
            AddressingMode::Absolute => {
                let addr = cpu.read_16(pc + 1);
                (addr, false)
            }

            AddressingMode::AbsoluteX => {
                let addr = cpu.read_16(pc + 1);
                let new_addr = addr.wrapping_add(cpu.x as u16);
                (new_addr, self.page_crossed(addr, new_addr))
            }

            AddressingMode::AbsoluteY => {
                let addr = cpu.read_16(pc + 1);
                let new_addr = addr.wrapping_add(cpu.y as u16);
                (new_addr, self.page_crossed(addr, new_addr))
            }

            //// Indirect ////
            AddressingMode::Indirect => {
                let addr = cpu.read_16(pc + 1);
                let addr = cpu.read_16_from_same_page(addr);
                (addr, false)
            }

            AddressingMode::IndirectX => {
                let addr = cpu.read(pc + 1).wrapping_add(cpu.x) as u16;
                let addr = cpu.read_16_from_same_page(addr);
                (addr, false)
            }

            AddressingMode::IndirectY => {
                let addr = cpu.read(pc + 1) as u16;
                let new_addr = cpu.read_16_from_same_page(addr).wrapping_add(cpu.y as u16);
                (new_addr, self.page_crossed(addr, new_addr))
            }
        }
    }

    fn page_crossed(&self, addr_1: u16, addr_2: u16) -> bool {
        (addr_1 & 0xFF00) != (addr_2 & 0xFF00)
    }
}

//// Instructions /////////////////////////////
pub enum Instruction {
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
    pub fn execute(&self, cpu: &mut CPU, addr: u16, mode: &AddressingMode) {
        match *self {
            //// BITWISE ////
            Instruction::AND => cpu.and(addr),
            Instruction::EOR => cpu.eor(addr),
            Instruction::ORA => cpu.ora(addr),

            Instruction::ASL => cpu.asl(addr, mode),
            Instruction::LSR => cpu.lsr(addr, mode),
            Instruction::ROL => cpu.rol(addr, mode),
            Instruction::ROR => cpu.ror(addr, mode),

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
            Instruction::JMP => cpu.jmp(addr),
            Instruction::JSR => cpu.jsr(addr),
            Instruction::RTI => cpu.rti(),
            Instruction::RTS => cpu.rts(),

            //// MATH ////
            Instruction::ADC => cpu.adc(addr),
            Instruction::SBC => cpu.sbc(addr),

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
            Instruction::PHA => cpu.pha(),
            Instruction::PHP => cpu.php(),
            Instruction::PLA => cpu.pla(),
            Instruction::PLP => cpu.plp(),

            //// OTHER ////
            Instruction::NOP => (),
            Instruction::BRK => cpu.brk(),

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
        }
    }
}

//// OPCODES /////////////////////////////////
use AddressingMode::*;
use Instruction::*;

pub struct OPCODE {
    pub instruction: Instruction,
    pub mode: AddressingMode,
    pub size: u16,
    pub cycles: u32,
    pub extra_cycles: u32,
}

#[rustfmt::skip]
pub const OPCODES: [OPCODE; 256] = [
    OPCODE { instruction: BRK, mode: Implied,     size: 1, cycles: 7, extra_cycles: 0 },
    OPCODE { instruction: ORA, mode: IndirectX,   size: 2, cycles: 6, extra_cycles: 0 },
    OPCODE { instruction: JAM, mode: Implied,     size: 1, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: SLO, mode: IndirectX,   size: 2, cycles: 8, extra_cycles: 0 },
    OPCODE { instruction: NOP, mode: ZeroPage,    size: 2, cycles: 3, extra_cycles: 0 },
    OPCODE { instruction: ORA, mode: ZeroPage,    size: 2, cycles: 3, extra_cycles: 0 },
    OPCODE { instruction: ASL, mode: ZeroPage,    size: 2, cycles: 5, extra_cycles: 0 },
    OPCODE { instruction: SLO, mode: ZeroPage,    size: 2, cycles: 5, extra_cycles: 0 },
    OPCODE { instruction: PHP, mode: Implied,     size: 1, cycles: 3, extra_cycles: 0 },
    OPCODE { instruction: ORA, mode: Immediate,   size: 2, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: ASL, mode: Accumulator, size: 1, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: ANC, mode: Immediate,   size: 2, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: NOP, mode: Absolute,    size: 3, cycles: 4, extra_cycles: 0 },
    OPCODE { instruction: ORA, mode: Absolute,    size: 3, cycles: 4, extra_cycles: 0 },
    OPCODE { instruction: ASL, mode: Absolute,    size: 3, cycles: 6, extra_cycles: 0 },
    OPCODE { instruction: SLO, mode: Absolute,    size: 3, cycles: 6, extra_cycles: 0 },
    OPCODE { instruction: BPL, mode: Relative,    size: 2, cycles: 2, extra_cycles: 1 },
    OPCODE { instruction: ORA, mode: IndirectY,   size: 2, cycles: 5, extra_cycles: 1 },
    OPCODE { instruction: JAM, mode: Implied,     size: 1, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: SLO, mode: IndirectY,   size: 2, cycles: 8, extra_cycles: 0 },
    OPCODE { instruction: NOP, mode: ZeroPageX,   size: 2, cycles: 4, extra_cycles: 0 },
    OPCODE { instruction: ORA, mode: ZeroPageX,   size: 2, cycles: 4, extra_cycles: 0 },
    OPCODE { instruction: ASL, mode: ZeroPageX,   size: 2, cycles: 6, extra_cycles: 0 },
    OPCODE { instruction: SLO, mode: ZeroPageX,   size: 2, cycles: 6, extra_cycles: 0 },
    OPCODE { instruction: CLC, mode: Implied,     size: 1, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: ORA, mode: AbsoluteY,   size: 3, cycles: 4, extra_cycles: 1 },
    OPCODE { instruction: NOP, mode: Implied,     size: 1, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: SLO, mode: AbsoluteY,   size: 3, cycles: 7, extra_cycles: 0 },
    OPCODE { instruction: NOP, mode: AbsoluteX,   size: 3, cycles: 4, extra_cycles: 1 },
    OPCODE { instruction: ORA, mode: AbsoluteX,   size: 3, cycles: 4, extra_cycles: 1 },
    OPCODE { instruction: ASL, mode: AbsoluteX,   size: 3, cycles: 7, extra_cycles: 0 },
    OPCODE { instruction: SLO, mode: AbsoluteX,   size: 3, cycles: 7, extra_cycles: 0 },
    OPCODE { instruction: JSR, mode: Absolute,    size: 3, cycles: 6, extra_cycles: 0 },
    OPCODE { instruction: AND, mode: IndirectX,   size: 2, cycles: 6, extra_cycles: 0 },
    OPCODE { instruction: JAM, mode: Implied,     size: 1, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: RLA, mode: IndirectX,   size: 2, cycles: 8, extra_cycles: 0 },
    OPCODE { instruction: BIT, mode: ZeroPage,    size: 2, cycles: 3, extra_cycles: 0 },
    OPCODE { instruction: AND, mode: ZeroPage,    size: 2, cycles: 3, extra_cycles: 0 },
    OPCODE { instruction: ROL, mode: ZeroPage,    size: 2, cycles: 5, extra_cycles: 0 },
    OPCODE { instruction: RLA, mode: ZeroPage,    size: 2, cycles: 5, extra_cycles: 0 },
    OPCODE { instruction: PLP, mode: Implied,     size: 1, cycles: 4, extra_cycles: 0 },
    OPCODE { instruction: AND, mode: Immediate,   size: 2, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: ROL, mode: Accumulator, size: 1, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: ANC, mode: Immediate,   size: 2, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: BIT, mode: Absolute,    size: 3, cycles: 4, extra_cycles: 0 },
    OPCODE { instruction: AND, mode: Absolute,    size: 3, cycles: 4, extra_cycles: 0 },
    OPCODE { instruction: ROL, mode: Absolute,    size: 3, cycles: 6, extra_cycles: 0 },
    OPCODE { instruction: RLA, mode: Absolute,    size: 3, cycles: 6, extra_cycles: 0 },
    OPCODE { instruction: BMI, mode: Relative,    size: 2, cycles: 2, extra_cycles: 1 },
    OPCODE { instruction: AND, mode: IndirectY,   size: 2, cycles: 5, extra_cycles: 1 },
    OPCODE { instruction: JAM, mode: Implied,     size: 1, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: RLA, mode: IndirectY,   size: 2, cycles: 8, extra_cycles: 0 },
    OPCODE { instruction: NOP, mode: ZeroPageX,   size: 2, cycles: 4, extra_cycles: 0 },
    OPCODE { instruction: AND, mode: ZeroPageX,   size: 2, cycles: 4, extra_cycles: 0 },
    OPCODE { instruction: ROL, mode: ZeroPageX,   size: 2, cycles: 6, extra_cycles: 0 },
    OPCODE { instruction: RLA, mode: ZeroPageX,   size: 2, cycles: 6, extra_cycles: 0 },
    OPCODE { instruction: SEC, mode: Implied,     size: 1, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: AND, mode: AbsoluteY,   size: 3, cycles: 4, extra_cycles: 1 },
    OPCODE { instruction: NOP, mode: Implied,     size: 1, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: RLA, mode: AbsoluteY,   size: 3, cycles: 7, extra_cycles: 0 },
    OPCODE { instruction: NOP, mode: AbsoluteX,   size: 3, cycles: 4, extra_cycles: 1 },
    OPCODE { instruction: AND, mode: AbsoluteX,   size: 3, cycles: 4, extra_cycles: 1 },
    OPCODE { instruction: ROL, mode: AbsoluteX,   size: 3, cycles: 7, extra_cycles: 0 },
    OPCODE { instruction: RLA, mode: AbsoluteX,   size: 3, cycles: 7, extra_cycles: 0 },
    OPCODE { instruction: RTI, mode: Implied,     size: 1, cycles: 6, extra_cycles: 0 },
    OPCODE { instruction: EOR, mode: IndirectX,   size: 2, cycles: 6, extra_cycles: 0 },
    OPCODE { instruction: JAM, mode: Implied,     size: 1, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: SRE, mode: IndirectX,   size: 2, cycles: 8, extra_cycles: 0 },
    OPCODE { instruction: NOP, mode: ZeroPage,    size: 2, cycles: 3, extra_cycles: 0 },
    OPCODE { instruction: EOR, mode: ZeroPage,    size: 2, cycles: 3, extra_cycles: 0 },
    OPCODE { instruction: LSR, mode: ZeroPage,    size: 2, cycles: 5, extra_cycles: 0 },
    OPCODE { instruction: SRE, mode: ZeroPage,    size: 2, cycles: 5, extra_cycles: 0 },
    OPCODE { instruction: PHA, mode: Implied,     size: 1, cycles: 3, extra_cycles: 0 },
    OPCODE { instruction: EOR, mode: Immediate,   size: 2, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: LSR, mode: Accumulator, size: 1, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: ALR, mode: Immediate,   size: 2, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: JMP, mode: Absolute,    size: 3, cycles: 3, extra_cycles: 0 },
    OPCODE { instruction: EOR, mode: Absolute,    size: 3, cycles: 4, extra_cycles: 0 },
    OPCODE { instruction: LSR, mode: Absolute,    size: 3, cycles: 6, extra_cycles: 0 },
    OPCODE { instruction: SRE, mode: Absolute,    size: 3, cycles: 6, extra_cycles: 0 },
    OPCODE { instruction: BVC, mode: Relative,    size: 2, cycles: 2, extra_cycles: 1 },
    OPCODE { instruction: EOR, mode: IndirectY,   size: 2, cycles: 5, extra_cycles: 1 },
    OPCODE { instruction: JAM, mode: Implied,     size: 1, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: SRE, mode: IndirectY,   size: 2, cycles: 8, extra_cycles: 0 },
    OPCODE { instruction: NOP, mode: ZeroPageX,   size: 2, cycles: 4, extra_cycles: 0 },
    OPCODE { instruction: EOR, mode: ZeroPageX,   size: 2, cycles: 4, extra_cycles: 0 },
    OPCODE { instruction: LSR, mode: ZeroPageX,   size: 2, cycles: 6, extra_cycles: 0 },
    OPCODE { instruction: SRE, mode: ZeroPageX,   size: 2, cycles: 6, extra_cycles: 0 },
    OPCODE { instruction: CLI, mode: Implied,     size: 1, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: EOR, mode: AbsoluteY,   size: 3, cycles: 4, extra_cycles: 1 },
    OPCODE { instruction: NOP, mode: Implied,     size: 1, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: SRE, mode: AbsoluteY,   size: 3, cycles: 7, extra_cycles: 0 },
    OPCODE { instruction: NOP, mode: AbsoluteX,   size: 3, cycles: 4, extra_cycles: 1 },
    OPCODE { instruction: EOR, mode: AbsoluteX,   size: 3, cycles: 4, extra_cycles: 1 },
    OPCODE { instruction: LSR, mode: AbsoluteX,   size: 3, cycles: 7, extra_cycles: 0 },
    OPCODE { instruction: SRE, mode: AbsoluteX,   size: 3, cycles: 7, extra_cycles: 0 },
    OPCODE { instruction: RTS, mode: Implied,     size: 1, cycles: 6, extra_cycles: 0 },
    OPCODE { instruction: ADC, mode: IndirectX,   size: 2, cycles: 6, extra_cycles: 0 },
    OPCODE { instruction: JAM, mode: Implied,     size: 1, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: RRA, mode: IndirectX,   size: 2, cycles: 8, extra_cycles: 0 },
    OPCODE { instruction: NOP, mode: ZeroPage,    size: 2, cycles: 3, extra_cycles: 0 },
    OPCODE { instruction: ADC, mode: ZeroPage,    size: 2, cycles: 3, extra_cycles: 0 },
    OPCODE { instruction: ROR, mode: ZeroPage,    size: 2, cycles: 5, extra_cycles: 0 },
    OPCODE { instruction: RRA, mode: ZeroPage,    size: 2, cycles: 5, extra_cycles: 0 },
    OPCODE { instruction: PLA, mode: Implied,     size: 1, cycles: 4, extra_cycles: 0 },
    OPCODE { instruction: ADC, mode: Immediate,   size: 2, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: ROR, mode: Accumulator, size: 1, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: ARR, mode: Immediate,   size: 2, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: JMP, mode: Indirect,    size: 3, cycles: 5, extra_cycles: 0 },
    OPCODE { instruction: ADC, mode: Absolute,    size: 3, cycles: 4, extra_cycles: 0 },
    OPCODE { instruction: ROR, mode: Absolute,    size: 3, cycles: 6, extra_cycles: 0 },
    OPCODE { instruction: RRA, mode: Absolute,    size: 3, cycles: 6, extra_cycles: 0 },
    OPCODE { instruction: BVS, mode: Relative,    size: 2, cycles: 2, extra_cycles: 1 },
    OPCODE { instruction: ADC, mode: IndirectY,   size: 2, cycles: 5, extra_cycles: 1 },
    OPCODE { instruction: JAM, mode: Implied,     size: 1, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: RRA, mode: IndirectY,   size: 2, cycles: 8, extra_cycles: 0 },
    OPCODE { instruction: NOP, mode: ZeroPageX,   size: 2, cycles: 4, extra_cycles: 0 },
    OPCODE { instruction: ADC, mode: ZeroPageX,   size: 2, cycles: 4, extra_cycles: 0 },
    OPCODE { instruction: ROR, mode: ZeroPageX,   size: 2, cycles: 6, extra_cycles: 0 },
    OPCODE { instruction: RRA, mode: ZeroPageX,   size: 2, cycles: 6, extra_cycles: 0 },
    OPCODE { instruction: SEI, mode: Implied,     size: 1, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: ADC, mode: AbsoluteY,   size: 3, cycles: 4, extra_cycles: 1 },
    OPCODE { instruction: NOP, mode: Implied,     size: 1, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: RRA, mode: AbsoluteY,   size: 3, cycles: 7, extra_cycles: 0 },
    OPCODE { instruction: NOP, mode: AbsoluteX,   size: 3, cycles: 4, extra_cycles: 1 },
    OPCODE { instruction: ADC, mode: AbsoluteX,   size: 3, cycles: 4, extra_cycles: 1 },
    OPCODE { instruction: ROR, mode: AbsoluteX,   size: 3, cycles: 7, extra_cycles: 0 },
    OPCODE { instruction: RRA, mode: AbsoluteX,   size: 3, cycles: 7, extra_cycles: 0 },
    OPCODE { instruction: NOP, mode: Immediate,   size: 2, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: STA, mode: IndirectX,   size: 2, cycles: 6, extra_cycles: 0 },
    OPCODE { instruction: NOP, mode: Immediate,   size: 2, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: SAX, mode: IndirectX,   size: 2, cycles: 6, extra_cycles: 0 },
    OPCODE { instruction: STY, mode: ZeroPage,    size: 2, cycles: 3, extra_cycles: 0 },
    OPCODE { instruction: STA, mode: ZeroPage,    size: 2, cycles: 3, extra_cycles: 0 },
    OPCODE { instruction: STX, mode: ZeroPage,    size: 2, cycles: 3, extra_cycles: 0 },
    OPCODE { instruction: SAX, mode: ZeroPage,    size: 2, cycles: 3, extra_cycles: 0 },
    OPCODE { instruction: DEY, mode: Implied,     size: 1, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: NOP, mode: Immediate,   size: 2, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: TXA, mode: Implied,     size: 1, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: XAA, mode: Immediate,   size: 2, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: STY, mode: Absolute,    size: 3, cycles: 4, extra_cycles: 0 },
    OPCODE { instruction: STA, mode: Absolute,    size: 3, cycles: 4, extra_cycles: 0 },
    OPCODE { instruction: STX, mode: Absolute,    size: 3, cycles: 4, extra_cycles: 0 },
    OPCODE { instruction: SAX, mode: Absolute,    size: 3, cycles: 4, extra_cycles: 0 },
    OPCODE { instruction: BCC, mode: Relative,    size: 2, cycles: 2, extra_cycles: 1 },
    OPCODE { instruction: STA, mode: IndirectY,   size: 2, cycles: 6, extra_cycles: 0 },
    OPCODE { instruction: JAM, mode: Implied,     size: 1, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: AHX, mode: IndirectY,   size: 2, cycles: 6, extra_cycles: 0 },
    OPCODE { instruction: STY, mode: ZeroPageX,   size: 2, cycles: 4, extra_cycles: 0 },
    OPCODE { instruction: STA, mode: ZeroPageX,   size: 2, cycles: 4, extra_cycles: 0 },
    OPCODE { instruction: STX, mode: ZeroPageY,   size: 2, cycles: 4, extra_cycles: 0 },
    OPCODE { instruction: SAX, mode: ZeroPageY,   size: 2, cycles: 4, extra_cycles: 0 },
    OPCODE { instruction: TYA, mode: Implied,     size: 1, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: STA, mode: AbsoluteY,   size: 3, cycles: 5, extra_cycles: 0 },
    OPCODE { instruction: TXS, mode: Implied,     size: 1, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: TAS, mode: AbsoluteY,   size: 3, cycles: 5, extra_cycles: 0 },
    OPCODE { instruction: SHY, mode: AbsoluteX,   size: 3, cycles: 5, extra_cycles: 0 },
    OPCODE { instruction: STA, mode: AbsoluteX,   size: 3, cycles: 5, extra_cycles: 0 },
    OPCODE { instruction: SHX, mode: AbsoluteY,   size: 3, cycles: 5, extra_cycles: 0 },
    OPCODE { instruction: AHX, mode: AbsoluteY,   size: 3, cycles: 5, extra_cycles: 0 },
    OPCODE { instruction: LDY, mode: Immediate,   size: 2, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: LDA, mode: IndirectX,   size: 2, cycles: 6, extra_cycles: 0 },
    OPCODE { instruction: LDX, mode: Immediate,   size: 2, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: LAX, mode: IndirectX,   size: 2, cycles: 6, extra_cycles: 0 },
    OPCODE { instruction: LDY, mode: ZeroPage,    size: 2, cycles: 3, extra_cycles: 0 },
    OPCODE { instruction: LDA, mode: ZeroPage,    size: 2, cycles: 3, extra_cycles: 0 },
    OPCODE { instruction: LDX, mode: ZeroPage,    size: 2, cycles: 3, extra_cycles: 0 },
    OPCODE { instruction: LAX, mode: ZeroPage,    size: 2, cycles: 3, extra_cycles: 0 },
    OPCODE { instruction: TAY, mode: Implied,     size: 1, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: LDA, mode: Immediate,   size: 2, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: TAX, mode: Implied,     size: 1, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: LAX, mode: Immediate,   size: 2, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: LDY, mode: Absolute,    size: 3, cycles: 4, extra_cycles: 0 },
    OPCODE { instruction: LDA, mode: Absolute,    size: 3, cycles: 4, extra_cycles: 0 },
    OPCODE { instruction: LDX, mode: Absolute,    size: 3, cycles: 4, extra_cycles: 0 },
    OPCODE { instruction: LAX, mode: Absolute,    size: 3, cycles: 4, extra_cycles: 0 },
    OPCODE { instruction: BCS, mode: Relative,    size: 2, cycles: 2, extra_cycles: 1 },
    OPCODE { instruction: LDA, mode: IndirectY,   size: 2, cycles: 5, extra_cycles: 1 },
    OPCODE { instruction: JAM, mode: Implied,     size: 1, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: LAX, mode: IndirectY,   size: 2, cycles: 5, extra_cycles: 1 },
    OPCODE { instruction: LDY, mode: ZeroPageX,   size: 2, cycles: 4, extra_cycles: 0 },
    OPCODE { instruction: LDA, mode: ZeroPageX,   size: 2, cycles: 4, extra_cycles: 0 },
    OPCODE { instruction: LDX, mode: ZeroPageY,   size: 2, cycles: 4, extra_cycles: 0 },
    OPCODE { instruction: LAX, mode: ZeroPageY,   size: 2, cycles: 4, extra_cycles: 0 },
    OPCODE { instruction: CLV, mode: Implied,     size: 1, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: LDA, mode: AbsoluteY,   size: 3, cycles: 4, extra_cycles: 1 },
    OPCODE { instruction: TSX, mode: Implied,     size: 1, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: LAS, mode: AbsoluteY,   size: 3, cycles: 4, extra_cycles: 1 },
    OPCODE { instruction: LDY, mode: AbsoluteX,   size: 3, cycles: 4, extra_cycles: 1 },
    OPCODE { instruction: LDA, mode: AbsoluteX,   size: 3, cycles: 4, extra_cycles: 1 },
    OPCODE { instruction: LDX, mode: AbsoluteY,   size: 3, cycles: 4, extra_cycles: 1 },
    OPCODE { instruction: LAX, mode: AbsoluteY,   size: 3, cycles: 4, extra_cycles: 1 },
    OPCODE { instruction: CPY, mode: Immediate,   size: 2, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: CMP, mode: IndirectX,   size: 2, cycles: 6, extra_cycles: 0 },
    OPCODE { instruction: NOP, mode: Immediate,   size: 2, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: DCP, mode: IndirectX,   size: 2, cycles: 8, extra_cycles: 0 },
    OPCODE { instruction: CPY, mode: ZeroPage,    size: 2, cycles: 3, extra_cycles: 0 },
    OPCODE { instruction: CMP, mode: ZeroPage,    size: 2, cycles: 3, extra_cycles: 0 },
    OPCODE { instruction: DEC, mode: ZeroPage,    size: 2, cycles: 5, extra_cycles: 0 },
    OPCODE { instruction: DCP, mode: ZeroPage,    size: 2, cycles: 5, extra_cycles: 0 },
    OPCODE { instruction: INY, mode: Implied,     size: 1, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: CMP, mode: Immediate,   size: 2, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: DEX, mode: Implied,     size: 1, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: AXS, mode: Immediate,   size: 2, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: CPY, mode: Absolute,    size: 3, cycles: 4, extra_cycles: 0 },
    OPCODE { instruction: CMP, mode: Absolute,    size: 3, cycles: 4, extra_cycles: 0 },
    OPCODE { instruction: DEC, mode: Absolute,    size: 3, cycles: 6, extra_cycles: 0 },
    OPCODE { instruction: DCP, mode: Absolute,    size: 3, cycles: 6, extra_cycles: 0 },
    OPCODE { instruction: BNE, mode: Relative,    size: 2, cycles: 2, extra_cycles: 1 },
    OPCODE { instruction: CMP, mode: IndirectY,   size: 2, cycles: 5, extra_cycles: 1 },
    OPCODE { instruction: JAM, mode: Implied,     size: 1, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: DCP, mode: IndirectY,   size: 2, cycles: 8, extra_cycles: 0 },
    OPCODE { instruction: NOP, mode: ZeroPageX,   size: 2, cycles: 4, extra_cycles: 0 },
    OPCODE { instruction: CMP, mode: ZeroPageX,   size: 2, cycles: 4, extra_cycles: 0 },
    OPCODE { instruction: DEC, mode: ZeroPageX,   size: 2, cycles: 6, extra_cycles: 0 },
    OPCODE { instruction: DCP, mode: ZeroPageX,   size: 2, cycles: 6, extra_cycles: 0 },
    OPCODE { instruction: CLD, mode: Implied,     size: 1, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: CMP, mode: AbsoluteY,   size: 3, cycles: 4, extra_cycles: 1 },
    OPCODE { instruction: NOP, mode: Implied,     size: 1, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: DCP, mode: AbsoluteY,   size: 3, cycles: 7, extra_cycles: 0 },
    OPCODE { instruction: NOP, mode: AbsoluteX,   size: 3, cycles: 4, extra_cycles: 1 },
    OPCODE { instruction: CMP, mode: AbsoluteX,   size: 3, cycles: 4, extra_cycles: 1 },
    OPCODE { instruction: DEC, mode: AbsoluteX,   size: 3, cycles: 7, extra_cycles: 0 },
    OPCODE { instruction: DCP, mode: AbsoluteX,   size: 3, cycles: 7, extra_cycles: 0 },
    OPCODE { instruction: CPX, mode: Immediate,   size: 2, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: SBC, mode: IndirectX,   size: 2, cycles: 6, extra_cycles: 0 },
    OPCODE { instruction: NOP, mode: Immediate,   size: 2, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: ISB, mode: IndirectX,   size: 2, cycles: 8, extra_cycles: 0 },
    OPCODE { instruction: CPX, mode: ZeroPage,    size: 2, cycles: 3, extra_cycles: 0 },
    OPCODE { instruction: SBC, mode: ZeroPage,    size: 2, cycles: 3, extra_cycles: 0 },
    OPCODE { instruction: INC, mode: ZeroPage,    size: 2, cycles: 5, extra_cycles: 0 },
    OPCODE { instruction: ISB, mode: ZeroPage,    size: 2, cycles: 5, extra_cycles: 0 },
    OPCODE { instruction: INX, mode: Implied,     size: 1, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: SBC, mode: Immediate,   size: 2, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: NOP, mode: Implied,     size: 1, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: SBC, mode: Immediate,   size: 2, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: CPX, mode: Absolute,    size: 3, cycles: 4, extra_cycles: 0 },
    OPCODE { instruction: SBC, mode: Absolute,    size: 3, cycles: 4, extra_cycles: 0 },
    OPCODE { instruction: INC, mode: Absolute,    size: 3, cycles: 6, extra_cycles: 0 },
    OPCODE { instruction: ISB, mode: Absolute,    size: 3, cycles: 6, extra_cycles: 0 },
    OPCODE { instruction: BEQ, mode: Relative,    size: 2, cycles: 2, extra_cycles: 1 },
    OPCODE { instruction: SBC, mode: IndirectY,   size: 2, cycles: 5, extra_cycles: 1 },
    OPCODE { instruction: JAM, mode: Implied,     size: 1, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: ISB, mode: IndirectY,   size: 2, cycles: 8, extra_cycles: 0 },
    OPCODE { instruction: NOP, mode: ZeroPageX,   size: 2, cycles: 4, extra_cycles: 0 },
    OPCODE { instruction: SBC, mode: ZeroPageX,   size: 2, cycles: 4, extra_cycles: 0 },
    OPCODE { instruction: INC, mode: ZeroPageX,   size: 2, cycles: 6, extra_cycles: 0 },
    OPCODE { instruction: ISB, mode: ZeroPageX,   size: 2, cycles: 6, extra_cycles: 0 },
    OPCODE { instruction: SED, mode: Implied,     size: 1, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: SBC, mode: AbsoluteY,   size: 3, cycles: 4, extra_cycles: 1 },
    OPCODE { instruction: NOP, mode: Implied,     size: 1, cycles: 2, extra_cycles: 0 },
    OPCODE { instruction: ISB, mode: AbsoluteY,   size: 3, cycles: 7, extra_cycles: 0 },
    OPCODE { instruction: NOP, mode: AbsoluteX,   size: 3, cycles: 4, extra_cycles: 1 },
    OPCODE { instruction: SBC, mode: AbsoluteX,   size: 3, cycles: 4, extra_cycles: 1 },
    OPCODE { instruction: INC, mode: AbsoluteX,   size: 3, cycles: 7, extra_cycles: 0 },
    OPCODE { instruction: ISB, mode: AbsoluteX,   size: 3, cycles: 7, extra_cycles: 0 },
];
