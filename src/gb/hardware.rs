use crate::gb::context::Context;

pub mod audio;
pub mod cpu;
pub mod graphics;
pub mod input;
pub mod memory;
pub mod serial;
pub mod timer;

pub trait HardwareInit {
    fn init(ctx: &mut Context);
}

pub trait MemoryInterface {
    fn read(ctx: &Context, address: u16) -> u8;
    fn write(ctx: &mut Context, address: u16, value: u8);
}

pub trait MTick {
    fn m_tick(ctx: &mut Context);
}
