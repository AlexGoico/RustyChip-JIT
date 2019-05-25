//! rcj is a CHIP-8 emulator with a JIT
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

pub mod logging;
pub mod render;
pub mod runtime;
pub mod settings;

use crate::settings::Settings;
use std::path::PathBuf;
use structopt::StructOpt;

/// Top level program arguments
#[derive(Debug, StructOpt)]
pub struct App {
    /// The CHIP-8 ROM to run
    rom_file: PathBuf,
}

fn main() {
    if let Err(e) = logging::setup_logger() {
        eprintln!("Error: failed to set up logging: {}", e);
        std::process::exit(-1);
    }
    info!("Test");
    debug!("Test");
    warn!("Test");
    error!("Test");
    trace!("Test");
    println!("Hello, world");

    let app = App::from_args();
    let settings = Settings::new();
    // load ROM
    // init chip8
    let mut render = render::Render::new(&settings);

    'program: loop {
        use render::EventCommand;
        let event_commands = render.handle_events();
        // execute logic

        for ec in event_commands {
            match ec {
                EventCommand::Quit => {
                    info!("Exiting!");
                    break 'program;
                }
                _ => (),
            }
        }
        // sleep
    }
}
