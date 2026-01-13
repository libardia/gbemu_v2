use crate::gb::regions::MemoryRegion;

macro_rules! reg_internal {
    ($name:ident, $address:literal) => {
        pub const $name: u16 = $address;
    };
    ($name:ident, $begin:literal, $end:literal) => {
        pub const $name: MemoryRegion = MemoryRegion::new($begin, $end);
    };
}

macro_rules! regs {
    {
        $(
            $name:ident: $begin:literal$(-$end:literal)?;
        )*
    } => {
        $(
            reg_internal!($name, $begin $(, $end)?);
        )*
    };
}

// Input
regs! {
    IO_JOYP: 0xFF00;
}

// Serial
regs! {
    ALL_SERIAL_REGS: 0xFF01 - 0xFF02;
    IO_SB: 0xFF01;
    IO_SC: 0xFF02;
}

// Timer
regs! {
    ALL_TIMER_REGS: 0xFF04 - 0xFF07;
    IO_DIV:  0xFF04;
    IO_TIMA: 0xFF05;
    IO_TMA:  0xFF06;
    IO_TAC:  0xFF07;
}

// Processor
regs! {
    IO_IF: 0xFF0F;
}

// Audio
regs! {
    ALL_AUDIO_REGS: 0xFF10 - 0xFF3F;

    IO_NR10: 0xFF10;
    IO_NR11: 0xFF11;
    IO_NR12: 0xFF12;
    IO_NR13: 0xFF13;
    IO_NR14: 0xFF14;

    IO_NR21: 0xFF16;
    IO_NR22: 0xFF17;
    IO_NR23: 0xFF18;
    IO_NR24: 0xFF19;

    IO_NR30: 0xFF1A;
    IO_NR31: 0xFF1B;
    IO_NR32: 0xFF1C;
    IO_NR33: 0xFF1D;
    IO_NR34: 0xFF1E;

    IO_NR41: 0xFF20;
    IO_NR42: 0xFF21;
    IO_NR43: 0xFF22;
    IO_NR44: 0xFF23;

    IO_NR50: 0xFF24;
    IO_NR51: 0xFF25;
    IO_NR52: 0xFF26;

    IO_WAVE: 0xFF30 - 0xFF3F;
}

// Graphics
regs! {
    ALL_GRAPHICS_REGS: 0xFF40 - 0xFF4B;
    IO_LCDC: 0xFF40;
    IO_STAT: 0xFF41;
    IO_SCY:  0xFF42;
    IO_SCX:  0xFF43;
    IO_LY:   0xFF44;
    IO_LYC:  0xFF45;
    IO_DMA:  0xFF46;
    IO_BGP:  0xFF47;
    IO_OBP0: 0xFF48;
    IO_OBP1: 0xFF49;
    IO_WY:   0xFF4A;
    IO_WX:   0xFF4B;
}

// Misc
regs! {
    IO_BANK: 0xFF50;
    IO_IE:   0xFFFF;
}
