use log::{debug, info};

use crate::gb::{context::Context, options::Options};

mod context;
mod hardware;
pub mod options;

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
