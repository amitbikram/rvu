use clap::Parser;
use image::ImageError;
use thiserror::Error;
use winit::{
    error::OsError,
    event::{ElementState, KeyboardInput, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[derive(Error, Debug)]
enum RvuError {
    #[error("Unable to create window")]
    WindowError(#[from] OsError),
    #[error("image error")]
    ImageError1(#[from] ImageError),
    #[error("io error")]
    IOError(#[from] std::io::Error),
}

type Result<T> = std::result::Result<T, RvuError>;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Config {
    file_name: String,
}

fn main() -> Result<()> {
    let config = Config::parse();
    let img = image::io::Reader::open(&config.file_name)?.decode()?;
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            winit::event::Event::WindowEvent { window_id, event } if window_id == window.id() => {
                match event {
                    winit::event::WindowEvent::Resized(_) => {}
                    winit::event::WindowEvent::CloseRequested => {}
                    winit::event::WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    _ => {}
                }
            }
            winit::event::Event::RedrawRequested(_) => {}
            _ => {}
        }
    });
}
