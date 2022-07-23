mod cpu;

use std::path::PathBuf;
use std::env;
use std::path::Path;
use cpu::Cpu;
use pixels::Pixels;
use pixels::SurfaceTexture;
use winit::event_loop::EventLoop;
use winit::window::Window;

const GRAPHICS_ROWS: u32 = 32;

const GRAPHICS_COLUMNS: u32 = 64;

fn main () {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();
    let size = window.inner_size();

    let surface_texture = SurfaceTexture::new(size.width, size.height, &window);

    let pixels = Pixels::new(GRAPHICS_COLUMNS, GRAPHICS_ROWS, surface_texture).expect("Something went wrong when initializing");
    let mut cpu = Cpu::new(pixels);
    match get_game() {
        Some(ref path) => cpu.load_rom(path),
        _ => return
    };
    let running = true;
    while running {
        cpu.cycle();
    }
}

fn get_game () -> Option<PathBuf> {
    if env::args().len() != 2 {
        println!("wrong");
        return None;
    }

    match env::args().nth(1) {
        Some(x) => match Path::new(&x) {
            x if x.is_file() == true  => Some(x.to_path_buf()),
            _ => None
        },
        _ => None
    }
}

