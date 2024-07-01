mod mapper0;
mod mapper2;
mod mapper4;

pub use mapper0::Mapper0;
pub use mapper2::Mapper2;
pub use mapper4::Mapper4;

use crate::{buffer, rom::ROM};
pub trait Mapper {
    fn read(&mut self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, val: u8);
    fn data(&self) -> &ROM;

    // only used in MMC3
    fn step(&mut self) {}

    fn irq_triggered(&mut self) -> bool {
        false
    }

    fn encode(&self, buffer: &mut buffer::Buffer);
    fn decode(&mut self, buffer: &mut buffer::Buffer);
}
