mod chip8;

use chip8::Chip8;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use std::env;
use std::time::{Duration, Instant};

// # of CPU Cycles where input is debounced
static DEBOUNCE_DELAY: u8 = 25;
// Foreground Colour
static COLOUR_ACTIVE: u32 = 0x729FCF;
// Background Colour
static COLOUR_INACTIVE: u32 = 0x001A21;
// SDL2 Render Scale
static RENDER_SCALE: u32 = 10;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let file_path = match args.len() {
        2 => args[1].as_str().to_owned(),
        _ => {
            println!("Usage: cargo run --release [ch8 ROM file]");
            return;
        }
    };

    let sdl = sdl2::init().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();

    // Set up Display
    let video = sdl.video().unwrap();
    let window = video
        .window("Chip 8 Emulator", 64 * RENDER_SCALE, 32 * RENDER_SCALE)
        .allow_highdpi()
        .build()
        .unwrap();

    // Set up button debounce
    let mut debounce: [u8; 16] = [0; 16];

    // Init chip8 system
    let mut chip8 = Chip8::new();

    // Load game into memory
    chip8.load_rom(&file_path);

    // Emulation loop
    'running: loop {
        let instant = Instant::now();

        for x in 0..debounce.len() {
            if debounce[x] > 0 {
                debounce[x] -= 1;
            } else {
                chip8.keypad[x] = 0;
            }
        }
        // emulate one cycle
        chip8.emulate_cycle();

        // if draw flag is set, update the screen
        if chip8.draw_flag {
            draw(&window, &event_pump, chip8.display);
            chip8.draw_flag = false;
        }

        // determine key presses
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(x), ..
                } => {
                    let place = match_keycode_to_keypad(x);
                    if place < 0xFFFF {
                        chip8.keypad[place] = 1;
                        debounce[place] = DEBOUNCE_DELAY;
                    }
                }
                _ => {}
            }
        }

        // Set to 60 actions per second max.
        let elapsed = instant.elapsed().as_micros();
        if elapsed < 16 {
            std::thread::sleep(Duration::new(0, 1_000_000 * (16 - elapsed as u32)));
        }
    }
}

fn match_keycode_to_keypad(x: Keycode) -> usize {
    match x {
        Keycode::Num1 => 1,
        Keycode::Num2 => 2,
        Keycode::Num3 => 3,
        Keycode::Num4 => 0xC,
        Keycode::Q => 0xC,
        Keycode::W => 4,
        Keycode::E => 5,
        Keycode::R => 6,
        Keycode::A => 7,
        Keycode::S => 8,
        Keycode::D => 9,
        Keycode::F => 0xE,
        Keycode::Z => 0xA,
        Keycode::X => 0,
        Keycode::C => 0xB,
        Keycode::V => 0xF,
        _ => 0xFFFF // invalid
    }
}

fn draw(window: &sdl2::video::Window, event_pump: &sdl2::EventPump, image: [u8; 2048]) {
    let mut image_u8: [u8; 2048 * 3] = [0; 2048 * 3];

    for i in 0..2048 {
        if image[i] != 0 {
            image_u8[(i * 3) + 0] = ((COLOUR_ACTIVE & 0xFF_00_00) >> 16) as u8;
            image_u8[(i * 3) + 1] = ((COLOUR_ACTIVE & 0x00_FF_00) >> 8) as u8;
            image_u8[(i * 3) + 2] = (COLOUR_ACTIVE & 0x00_00_FF) as u8;
        } else {
            image_u8[(i * 3) + 0] = ((COLOUR_INACTIVE & 0xFF_00_00) >> 16) as u8;
            image_u8[(i * 3) + 1] = ((COLOUR_INACTIVE & 0x00_FF_00) >> 8) as u8;
            image_u8[(i * 3) + 2] = (COLOUR_INACTIVE & 0x00_00_FF) as u8;
        }
    }

    let frame_buffer =
        Surface::from_data(&mut image_u8, 64, 32, 64 * 3, PixelFormatEnum::RGB24).unwrap();

    let mut win = window.surface(&event_pump).unwrap();

    let src = Rect::new(0, 0, 64, 32);
    let dst = Rect::new(0, 0, 64 * RENDER_SCALE, 32 * RENDER_SCALE);
    frame_buffer.blit_scaled(src, &mut win, dst).unwrap();

    win.finish().unwrap();
}
