use std::{env, fs::File, io::Read};

use chip8::{SCREEN_HEIGHT, SCREEN_WIDTH, Chip8};
use sdl2::event::Event;


const SCALE: u32 = 20;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;

fn main() {
    let args: Vec<_> = env::args().collect();

    dbg!(&args);
    if args.len() != 2 {
        println!("not enough args. I ain't using clap");
    }


    let mut chip8 = Chip8::new();
    let mut rom = File::open(&args[1]).expect("unable to open file");
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).expect("unable to read file to buffer");
    chip8.load_rom_data(&buffer);



    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("CHIP 8", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for evt in event_pump.poll_iter(){
            match evt {
                Event::Quit{..} => {
                    break 'running;
                }
                _ => {}
            }
        }
    }

}
