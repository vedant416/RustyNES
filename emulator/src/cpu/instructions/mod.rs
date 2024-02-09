// use super::AddressingMode;
use super::CPU;

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
    ///// IO utils //////////////////////////////
    fn read(&mut self, addr: u16) -> u8 {
        todo!("read");
    }

    fn read_16(&mut self, addr: u16) -> u16 {
        todo!("read_16");
    }

    fn read_16_from_same_page(&mut self, addr: u16) -> u16 {
        todo!("read_16_wrap");
    }

    fn write(&mut self, addr: u16, val: u8) {
        todo!("write");
    }

    ///// Stack utils ///////////////////////////
    fn push_8(&mut self, val: u8) {
        todo!("push_8");
    }

    fn pull_8(&mut self) -> u8 {
        todo!("pull_8");
    }

    fn push_16(&mut self, val: u16) {
        todo!("push_16");
    }

    fn pull_16(&mut self) -> u16 {
        todo!("pull_16");
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
        (addr_1 & 0xff00) != (addr_2 & 0xff00)
    }
}

//// Instructions /////////////////////////////
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
    pub fn execute(&self, cpu: &mut CPU, addr: u16, addr_mode: &AddressingMode) {
        match *self {
            //// BITWISE ////
            Instruction::AND => cpu.and(addr),
            Instruction::EOR => cpu.eor(addr),
            Instruction::ORA => cpu.ora(addr),

            Instruction::ASL => cpu.asl(addr, addr_mode),
            Instruction::LSR => cpu.lsr(addr, addr_mode),
            Instruction::ROL => cpu.rol(addr, addr_mode),
            Instruction::ROR => cpu.ror(addr, addr_mode),

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

