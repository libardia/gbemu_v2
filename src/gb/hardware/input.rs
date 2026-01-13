use derive_new::new;

use crate::gb::{
    context::Context,
    hardware::{HardwareInit, Tick, MemoryInterface},
};

#[derive(new)]
pub struct Input {}

impl HardwareInit for Input {
    fn init(ctx: &mut Context) {
        // TODO
        todo!()
    }
}

impl MemoryInterface for Input {
    fn read(ctx: &Context, address: u16) -> u8 {
        // TODO
        todo!()
    }

    fn write(ctx: &mut Context, address: u16, value: u8) {
        // TODO
        todo!()
    }
}

impl Tick for Input {
    fn tick(ctx: &mut Context) {
        // TODO
        todo!()
    }
}
