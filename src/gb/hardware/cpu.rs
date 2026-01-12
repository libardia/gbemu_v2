use crate::gb::{
    context::Context,
    hardware::{HardwareInit, MTick},
};

pub struct Processor {}

impl HardwareInit for Processor {
    fn init(ctx: &mut Context) {
        // TODO
        todo!()
    }
}

impl MTick for Processor {
    fn m_tick(ctx: &mut Context) {
        // TODO
        todo!()
    }
}
