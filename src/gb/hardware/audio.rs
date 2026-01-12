use crate::gb::{
    context::Context,
    hardware::{HardwareInit, MTick, MemoryInterface},
};

pub struct Audio {}

impl HardwareInit for Audio {
    fn init(ctx: &mut Context) {
        // TODO
        todo!()
    }
}

impl MemoryInterface for Audio {
    fn read(ctx: &Context, address: u16) -> u8 {
        // TODO
        todo!()
    }

    fn write(ctx: &mut Context, address: u16, value: u8) {
        // TODO
        todo!()
    }
}

impl MTick for Audio {
    fn m_tick(ctx: &mut Context) {
        // TODO
        todo!()
    }
}
