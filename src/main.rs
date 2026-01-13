#![allow(dead_code, unused_variables)]

use clap::Parser;
use log::{LevelFilter, debug, error, trace};
use std::{
    env,
    fs::{self, File},
    panic,
    path::Path,
    time::SystemTime,
};

use crate::gb::options::Options;

mod gb;
mod logging;

fn main() {
    init_logging("logs");

    // Log on panic instead of a simple print
    panic::set_hook(Box::new(|info| match info.location() {
        Some(loc) => error!(
            "Unrecoverable error at '{}' line {}; shutting down.",
            loc.file(),
            loc.line()
        ),
        None => error!("Unrecoverable error; shutting down."),
    }));

    // Parse commandline arguments
    let args: Vec<String> = env::args().collect();
    trace!("Raw args: {args:?}");
    let options = Options::parse();

    gb::run(options);
}

fn init_logging(base_dir: &str) {
    use fern::Dispatch;

    fs::create_dir(base_dir).ok();

    let console = Dispatch::new()
        .level(LevelFilter::Debug)
        .chain(std::io::stdout());

    let debug_file = Dispatch::new()
        .level(LevelFilter::Debug)
        .chain(log_file(base_dir, "debug.log"));

    let trace_file = Dispatch::new()
        .level(LevelFilter::Trace)
        .chain(log_file(base_dir, "trace.log"));

    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] [{}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .chain(console)
        .chain(debug_file)
        .chain(trace_file)
        .apply()
        .unwrap();
}

fn log_file(base_dir: &str, file_name: &str) -> File {
    std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .open(Path::new(base_dir).join(file_name))
        .unwrap()
}
