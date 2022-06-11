use std::{env, fs::File, io::Read};

use chip8::{Chip8, SCREEN_HEIGHT, SCREEN_WIDTH};
use rand::RngCore;
use sdl2::{event::Event, pixels::Color, rect::Rect, render::Canvas, video::Window};

const SCALE: u32 = 20;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const TICKS_PER_FRAME: usize = 10;

fn draw_screen<R: RngCore>(chip8: &Chip8<R>, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    let display = chip8.get_display();
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for (r, row) in display.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if *col == 1 {
                let rect = Rect::new(
                    (c as u32 * SCALE) as i32,
                    (r as u32 * SCALE) as i32,
                    SCALE,
                    SCALE,
                );
                canvas.fill_rect(rect).unwrap();
            }
        }
    }
    canvas.present();
}

fn main() {
    let args: Vec<_> = env::args().collect();

    dbg!(&args);
    if args.len() != 2 {
        println!("not enough args. I ain't using clap");
    }

    let mut chip8 = Chip8::new();
    let mut rom = File::open(&args[1]).expect("unable to open file");
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer)
        .expect("unable to read file to buffer");
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
        for evt in event_pump.poll_iter() {
            match evt {
                Event::Quit { .. } => {
                    break 'running;
                }
                _ => {}
            }

            for _ in 0..TICKS_PER_FRAME {
                chip8.tick();
            }
            chip8.timer_tick();
            draw_screen(&chip8, &mut canvas);
        }
    }
}
