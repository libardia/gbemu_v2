use crate::gb::{
    context::Context,
    hardware::{HardwareInit, MTick, MemoryInterface},
};

pub struct Graphics {}

impl HardwareInit for Graphics {
    fn init(ctx: &mut Context) {
        // TODO
        todo!()
    }
}

impl MemoryInterface for Graphics {
    fn read(ctx: &Context, address: u16) -> u8 {
        // TODO
        todo!()
    }

    fn write(ctx: &mut Context, address: u16, value: u8) {
        // TODO
        todo!()
    }
}

impl MTick for Graphics {
    fn m_tick(ctx: &mut Context) {
        // TODO
        todo!()
    }
}
