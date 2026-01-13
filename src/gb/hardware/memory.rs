use derive_new::new;
use log::warn;

use crate::{
    gb::{
        context::Context,
        hardware::{
            HardwareInit, MemoryInterface, address_dispatch, audio::Audio, cpu::Processor,
            graphics::Graphics, input::Input, serial::Serial, timer::Timer,
        },
        io_regs::{
            ALL_AUDIO_REGS, ALL_GRAPHICS_REGS, ALL_SERIAL_REGS, ALL_TIMER_REGS, IO_IF, IO_JOYP,
        },
        regions::{
            CART_RAM, ECHO_RAM, HIGH_RAM, MappedMemoryRegion, OAM, ROM_SPACE, VRAM, WORK_RAM,
        },
    },
    logging::{byte_fmt, word_fmt},
};

const ECHO_RAM_OFFSET: u16 = 0x2000;

#[derive(new)]
pub struct Memory {
    #[new(value = "MappedMemoryRegion::new(VRAM)")]
    vram: MappedMemoryRegion,

    #[new(value = "MappedMemoryRegion::new(WORK_RAM)")]
    wram: MappedMemoryRegion,

    #[new(value = "MappedMemoryRegion::new(OAM)")]
    oam: MappedMemoryRegion,

    #[new(value = "MappedMemoryRegion::new(HIGH_RAM)")]
    hram: MappedMemoryRegion,
}

impl HardwareInit for Memory {
    fn init(ctx: &mut Context) {
        // TODO
        todo!()
    }
}

impl MemoryInterface for Memory {
    fn read(ctx: &Context, address: u16) -> u8 {
        address_dispatch! {
            on address:
                // ROM
                #ROM_SPACE => todo!(), // TODO: Relay to cartridge

                // RAM
                #VRAM      => ctx.mem.vram.get(address),
                #CART_RAM  => todo!(), // TODO: Relay to cartridge
                #WORK_RAM  => ctx.mem.wram.get(address),
                #ECHO_RAM  => ctx.mem.wram.get(address - ECHO_RAM_OFFSET),
                #OAM       => ctx.mem.oam.get(address),
                #HIGH_RAM  => ctx.mem.hram.get(address), // Includes IE register

                // IO registers
                IO_JOYP            => Input::read(ctx, address),
                #ALL_SERIAL_REGS   => Serial::read(ctx, address),
                #ALL_TIMER_REGS    => Timer::read(ctx, address),
                IO_IF              => Processor::read(ctx, address),
                #ALL_AUDIO_REGS    => Audio::read(ctx, address),
                #ALL_GRAPHICS_REGS => Graphics::read(ctx, address),

                _ => {
                    warn!("Read from unreadable address {} returns {}", word_fmt!(address), byte_fmt!(0xFF));
                    0xFF
                },
        }
    }

    fn write(ctx: &mut Context, address: u16, value: u8) {
        address_dispatch! {
            on address:
                // ROM
                #ROM_SPACE => todo!(), // TODO: Relay to cartridge

                // RAM
                #VRAM      => ctx.mem.vram.set(address, value),
                #CART_RAM  => todo!(), // TODO: Relay to cartridge
                #WORK_RAM  => ctx.mem.wram.set(address, value),
                #ECHO_RAM  => ctx.mem.wram.set(address - ECHO_RAM_OFFSET, value),
                #OAM       => ctx.mem.oam.set(address, value),
                #HIGH_RAM  => ctx.mem.hram.set(address, value), // Includes IE register

                // IO registers
                IO_JOYP            => Input::write(ctx, address, value),
                #ALL_SERIAL_REGS   => Serial::write(ctx, address, value),
                #ALL_TIMER_REGS    => Timer::write(ctx, address, value),
                IO_IF              => Processor::write(ctx, address, value),
                #ALL_AUDIO_REGS    => Audio::write(ctx, address, value),
                #ALL_GRAPHICS_REGS => Graphics::write(ctx, address, value),

                _ => warn!("Ignored write of {} to unwritable address {}", byte_fmt!(value), word_fmt!(address)),
        }
    }
}
