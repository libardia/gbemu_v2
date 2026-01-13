use derive_new::new;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::{
    gb::{
        context::Context,
        hardware::{HardwareInit, MemoryInterface, Tick, address_dispatch},
        io_regs::{IO_DIV, IO_TAC, IO_TIMA, IO_TMA},
    },
    logging::warn_wrapping_add,
};

#[derive(Debug, Clone, Copy, FromPrimitive)]
enum ClockSpeed {
    Every256 = 0b00,
    Every4 = 0b01,
    Every16 = 0b10,
    Every64 = 0b11,
}

#[derive(new)]
pub struct Timer {
    #[new(value = "0")]
    system_timer: u16,

    #[new(value = "0")]
    timer_counter: u8,

    #[new(value = "0")]
    timer_modulo: u8,

    #[new(value = "false")]
    enable: bool,

    #[new(value = "ClockSpeed::Every256")]
    clock_speed: ClockSpeed,

    /// Used to emulate falling-edge detector, no matter how the timer changed
    #[new(value = "0")]
    system_timer_falling_edges: u16,
}

impl HardwareInit for Timer {
    fn init(ctx: &mut Context) {
        ctx.timer.system_timer = 0xAB00; // TODO: More accurate?
        ctx.timer.timer_counter = 0;
        ctx.timer.timer_modulo = 0;
        ctx.timer.enable = false;
        ctx.timer.clock_speed = ClockSpeed::Every256;
    }
}

impl MemoryInterface for Timer {
    fn read(ctx: &Context, address: u16) -> u8 {
        address_dispatch! {
            on address:
                IO_DIV  => (ctx.timer.system_timer >> 8) as u8,
                IO_TIMA => ctx.timer.timer_counter,
                IO_TMA  => ctx.timer.timer_modulo,
                IO_TAC  => ctx.timer.make_tac(),
        }
    }

    fn write(ctx: &mut Context, address: u16, value: u8) {
        address_dispatch! {
            on address:
                IO_DIV  => ctx.timer.system_timer = 0,
                IO_TIMA => ctx.timer.timer_counter = value,
                IO_TMA  => ctx.timer.timer_modulo = value,
                IO_TAC  => ctx.timer.unpack_tac(value),
        }
    }
}

impl Tick for Timer {
    fn tick(ctx: &mut Context) {
        // 1 m-cycle is 4 dots
        ctx.timer.set_system_timer(ctx.timer.system_timer + 4);

        // TODO: falling edges
    }
}

impl Timer {
    /// Sets the system timer, importantly recording resulting falling edges
    fn set_system_timer(&mut self, value: u16) {
        let before = self.system_timer;
        let after = value;

        self.system_timer = value;
        self.system_timer_falling_edges = (before ^ after) & !after;
        // TODO: resolve falling edges immediately?
    }

    /// Adds the given amount to the system timer, importantly recording resulting falling edges
    fn add_system_timer(&mut self, dt: u16) {
        self.set_system_timer(warn_wrapping_add!(self.system_timer, dt));
    }

    fn make_tac(&self) -> u8 {
        0b11111000 | (self.enable as u8) << 2 | (self.clock_speed as u8)
    }

    fn unpack_tac(&mut self, value: u8) {
        self.enable = (value & 0b100) != 0;
        self.clock_speed = ClockSpeed::from_u8(value & 0b11).unwrap();
    }
}
