mod cpu;

use std::path::PathBuf;
use std::env;
use std::path::Path;
use cpu::Cpu;


fn main() {
    let mut cpu = Cpu::new();
    match get_game() {
        Some(ref path) => cpu.load_rom(path),
        _ => return
    };
    let running = true;
    while running {
        cpu.cycle();
    }

}

fn get_game() -> Option<PathBuf> {
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

