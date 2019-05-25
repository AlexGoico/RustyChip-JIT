//! Rendering code.  Starting out simple with CPU-based rendering
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

pub struct Render {
    sdl_context: sdl2::Sdl,
    _video_subsystem: sdl2::VideoSubsystem,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

/// Commands given from the window's events
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EventCommand {
    /// End the program
    Quit,
}

impl Render {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let _video_subsystem = sdl_context.video().unwrap();
        let window = _video_subsystem
            .window(env!("CARGO_PKG_NAME"), 800, 600)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        Self {
            sdl_context,
            _video_subsystem,
            canvas,
        }
    }

    /// Draws a frame to the screen
    pub fn draw(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 255, 255));
        self.canvas.clear();
        self.canvas.present();
    }

    /// Handles window events and returns a vector of commands to execute
    pub fn handle_events(&mut self) -> Vec<EventCommand> {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        let mut out_events = vec![];
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => out_events.push(EventCommand::Quit),
                _ => {}
            }
        }

        out_events
    }
}
