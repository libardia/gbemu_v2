use derive_new::new;

use crate::gb::{
    hardware::{
        audio::Audio, cpu::Processor, graphics::Graphics, input::Input, memory::Memory,
        serial::Serial, timer::Timer,
    },
    options::Options,
};

#[derive(new)]
pub struct Context {
    pub options: Options,

    #[new(value = "Memory::new()")]
    pub mem: Memory,

    #[new(value = "Processor::new()")]
    pub cpu: Processor,

    #[new(value = "Graphics::new()")]
    pub gfx: Graphics,

    #[new(value = "Timer::new()")]
    pub timer: Timer,

    #[new(value = "Input::new()")]
    pub input: Input,

    #[new(value = "Audio::new()")]
    pub audio: Audio,

    #[new(value = "Serial::new()")]
    pub serial: Serial,

    #[new(value = "true")]
    pub running: bool,
}
