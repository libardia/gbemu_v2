use crate::gb::{
    hardware::{
        audio::Audio, cpu::Processor, graphics::Graphics, input::Input, memory::Memory,
        serial::Serial, timer::Timer,
    },
    options::Options,
};

pub struct Context {
    options: Options,

    mem: Memory,

    cpu: Processor,
    gfx: Graphics,
    timer: Timer,
    input: Input,
    audio: Audio,
    serial: Serial,
}

macro_rules! ctx {
    {
        $(
            $vis:vis fn $name:ident($($args:tt)*) $(-> $return:ty)? $body:block
        )*
    } => {
        $(
            $vis fn $name(ctx: &crate::gb::context::Context, $($args)*) $(-> $return)? $body
        )*
    }
}
pub(super) use ctx;

macro_rules! mctx {
    {
        $(
            $vis:vis fn $name:ident($($args:tt)*) $(-> $return:ty)? $body:block
        )*
    } => {
        $(
            $vis fn $name(ctx: &mut crate::gb::context::Context, $($args)*) $(-> $return)? $body
        )*
    }
}
pub(super) use mctx;
