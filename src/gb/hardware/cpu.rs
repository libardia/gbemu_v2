use derive_new::new;

use crate::gb::{
    context::Context,
    hardware::{HardwareInit, Tick},
};

#[derive(new)]
pub struct Processor {}

impl HardwareInit for Processor {
    fn init(ctx: &mut Context) {
        // TODO
        todo!()
    }
}

impl Tick for Processor {
    fn tick(ctx: &mut Context) {
        // TODO
        todo!()
    }
}
