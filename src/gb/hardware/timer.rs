use derive_new::new;
use log::trace;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::{
    gb::{
        context::Context,
        hardware::{HardwareInit, MemoryInterface, Tick, address_dispatch},
        io_regs::{IO_DIV, IO_TAC, IO_TIMA, IO_TMA},
    },
    logging::{byte_fmt, error_panic, warn_wrapping_add, word_fmt},
};

#[derive(Debug, Clone, Copy, FromPrimitive)]
enum ClockSpeed {
    Every256 = 0b00,
    Every4 = 0b01,
    Every16 = 0b10,
    Every64 = 0b11,
}

impl ClockSpeed {
    pub fn tick_bit(&self) -> u16 {
        match self {
            ClockSpeed::Every256 => 1 << 9,
            ClockSpeed::Every4 => 1 << 3,
            ClockSpeed::Every16 => 1 << 5,
            ClockSpeed::Every64 => 1 << 7,
        }
    }
}

const DIV_APU_BIT: u16 = 1 << 12;

#[derive(new)]
pub struct Timer {
    /// The true system timer underlying DIV
    #[new(value = "0")]
    system_timer: u16,

    /// TIMA
    #[new(value = "0")]
    timer_counter: u8,

    /// TMA
    #[new(value = "0")]
    timer_modulo: u8,

    /// TAC bit 2
    #[new(value = "false")]
    enable: bool,

    /// TAC bits 0-1
    #[new(value = "ClockSpeed::Every256")]
    clock_speed: ClockSpeed,

    /// Represents the bit selected by the timer multiplexer in real hardware
    #[new(value = "ClockSpeed::Every256.tick_bit()")]
    tick_bit: u16,
}

impl HardwareInit for Timer {
    fn init(ctx: &mut Context) {
        ctx.timer = Self {
            system_timer: 0xAB00, // TODO: More accurate?
            timer_counter: 0,
            timer_modulo: 0,
            enable: false,
            clock_speed: ClockSpeed::Every256,
            tick_bit: ClockSpeed::Every256.tick_bit(),
        }
    }
}

impl MemoryInterface for Timer {
    fn read(ctx: &Context, address: u16) -> u8 {
        address_dispatch! {
            on address:
                IO_DIV  => (ctx.timer.system_timer >> 8) as u8,
                IO_TIMA => ctx.timer.timer_counter,
                IO_TMA  => ctx.timer.timer_modulo,
                IO_TAC  => ctx.timer.pack_tac(),

                _ => error_panic!("Read from invalid address in timer: {}", word_fmt!(address)),
        }
    }

    fn write(ctx: &mut Context, address: u16, value: u8) {
        address_dispatch! {
            on address:
                IO_DIV  => Timer::maybe_causes_tick(ctx, |t| t.system_timer = 0),
                IO_TIMA => ctx.timer.timer_counter = value,
                IO_TMA  => ctx.timer.timer_modulo = value,
                IO_TAC  => Timer::maybe_causes_tick(ctx, |t| t.unpack_tac(value)),

                _ => error_panic!("Wrote {} to invalid address in timer: {}", byte_fmt!(value), word_fmt!(address)),
        }
    }
}

impl Tick for Timer {
    fn tick(ctx: &mut Context) {
        trace!("m-cycle tick: Timer");
        Timer::maybe_causes_tick(ctx, |t| {
            // 1 m-cycle is 4 dots
            t.system_timer = warn_wrapping_add!(t.system_timer, 4);
        });
    }
}

impl Timer {
    /* #region Associated functions */
    fn maybe_causes_tick<F>(ctx: &mut Context, action: F)
    where
        F: Fn(&mut Timer),
    {
        let tb_before = ctx.timer.timer_tick_bit();
        let da_before = ctx.timer.div_apu_bit();

        action(&mut ctx.timer);

        let tb_after = ctx.timer.timer_tick_bit();
        let da_after = ctx.timer.div_apu_bit();

        if tb_before && !tb_after {
            // No matter how anything changed, a falling edge between these two bools causes a timer
            // tick
            Timer::timer_tick(ctx);
        }

        if da_before && !da_after {
            // No matter how anything changed, a falling edge between these two bools causes a
            // DIV-APU event
            // TODO: DIV-APU event (to be filled out with Audio)
        }
    }

    fn timer_tick(ctx: &mut Context) {
        trace!("TIMA increment");
        let (result, overflow) = ctx.timer.timer_counter.overflowing_add(1);

        if overflow {
            ctx.timer.timer_counter = ctx.timer.timer_modulo;
            // TODO: request timer interrupt
        } else {
            ctx.timer.timer_counter = result;
        }
    }
    /* #endregion */

    /* #region Member functions */
    fn pack_tac(&self) -> u8 {
        0b11111000 | (self.enable as u8) << 2 | (self.clock_speed as u8)
    }

    fn unpack_tac(&mut self, value: u8) {
        self.enable = (value & 0b100) != 0;
        self.clock_speed = ClockSpeed::from_u8(value & 0b11).unwrap();
        self.tick_bit = self.clock_speed.tick_bit();
    }

    fn timer_tick_bit(&self) -> bool {
        // Emulates the multiplexer-TAC-enable circuit
        ((self.system_timer & self.tick_bit) != 0) && self.enable
    }

    fn div_apu_bit(&self) -> bool {
        // Emulates the DIV-APU circuit
        (self.system_timer & DIV_APU_BIT) != 0
    }
    /* #endregion */
}
