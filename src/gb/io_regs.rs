use crate::gb::regions::MemoryRegion;

macro_rules! reg_internal {
    ($name:ident: $address:literal) => {
        pub const $name: u16 = $address;
    };
    ($name:ident: $begin:literal-$end:literal) => {
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
            reg_internal!($name: $begin$(-$end)?);
        )*
    };
}

// Timer
regs! {
    IO_DIV:  0xFF04;
    IO_TIMA: 0xFF05;
    IO_TMA:  0xFF06;
    IO_TAC:  0xFF07;
}

// Audio
regs! {
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
