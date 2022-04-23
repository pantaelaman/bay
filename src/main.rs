use std::fs;
use log::{info, error};

mod base;
mod parser;

use crate::parser::parsebayfile;

fn start_logger() -> Result<(), fern::InitError> {
    let colors = fern::colors::ColoredLevelConfig::new()
        .info(fern::colors::Color::Green)
        .warn(fern::colors::Color::Yellow)
        .error(fern::colors::Color::Red)
        .trace(fern::colors::Color::White)
        .debug(fern::colors::Color::Magenta);
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                    "{color_line}{time}[{target}][{level}] {message}\x1B[0m",
                    color_line = format_args!(
                        "\x1B[{}m",
                        colors.get_color(&record.level()).to_fg_str()
                        ),
                    time = chrono::Local::now().format("[%H:%M:%S]"),
                    target = record.target(),
                    level = record.level(),
                    message = message,
                    ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

fn main() {
    match start_logger() {
        Ok(()) => info!(target: "logger", "Initialised logger"),
        Err(e) => println!("Could not initialise logger: {}", e),
    };
    // let input = "[target:all] fetch -git url:github.com; run \"git clone github.com\";";
    let input = match fs::read_to_string("example.bay") {
        Ok(s) => s,
        Err(e) => {error!(target: "input", "Could not read from file example.bay: {}", e); return},
    };
    let chunks = match parsebayfile(input.as_str()) {
        Ok((_, c)) => c,
        Err(e) => {error!(target: "parser", "Parsing of chunks failed: {}", e); return},
    };
    info!(target: "bay", "Produced chunks:\n{:#?}", chunks);
}

