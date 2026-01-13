use derive_new::new;

use crate::gb::{
    context::Context,
    hardware::{HardwareInit, MemoryInterface, Tick},
};

#[derive(new)]
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

impl Tick for Graphics {
    fn tick(ctx: &mut Context) {
        // TODO
        todo!()
    }
}
