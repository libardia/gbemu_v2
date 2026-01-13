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

pub trait Tick {
    fn tick(ctx: &mut Context);
}

macro_rules! address_dispatch {
    {
        on $address:ident:
            $($(#$reg:ident)?$($value:ident)? => $op:expr,)+
            $(_ => $op_other:expr,)?
    } => {
        match $address {
            $($(_ if $reg.contains($address))? $($value)? => $op,)+
            $(_ => $op_other,)?
        }
    };
}
pub(super) use address_dispatch;
