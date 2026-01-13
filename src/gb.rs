use log::{debug, info};

use crate::gb::{context::Context, options::Options};

mod context;
mod hardware;
mod io_regs;
pub mod options;
mod regions;

pub fn run(options: Options) {
    info!("Starting");

    let mut ctx = Context::new(options);
    debug!("Context created");

    while ctx.running {
        // TODO run
        ctx.running = false;
    }

    info!("Shutting down");
}
