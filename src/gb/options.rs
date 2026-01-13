use clap::Parser;
use getset::Getters;

#[derive(Parser, Getters, Debug)]
#[command(version, about, long_about = None)]
#[get = "pub"]
pub struct Options {
    /// If set, certain instructions that are otherwise invalid can be used for special functions.
    #[arg(short, long)]
    meta: bool,

    /// If set, the boot ROM is not skipped on startup.
    #[arg(short, long)]
    boot: bool,

    /// Path to the ROM file to open.
    rom_path: String,
}
