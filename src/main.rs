#![allow(dead_code)]
#![feature(core)]
#![feature(convert)] // Used for as_slice
#![feature(std_misc)] // Used for std::time::Duration
#![feature(thread_sleep)]

extern crate core;
extern crate sdl2;
extern crate sdl2_image;
extern crate time;

mod math;
mod world;
mod game;
mod input;
mod render;

use std::time::Duration;
use std::thread;
use std::path::Path;

use time::PreciseTime;

use sdl2::video::{Window, WindowPos, OPENGL};
use sdl2::mouse;
use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer, BlendMode};
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2_image::LoadSurface;


const WINDOW_WIDTH  :i32 = 2 * 320;
const WINDOW_HEIGHT :i32 = 2 * 240;

const W :usize = 320;
const H :usize = 240;


pub fn main() {
    let sdl_context = sdl2::init(sdl2::INIT_VIDEO).unwrap();

    let res_path = Path::new("/Users/jaburns/dev/rustycast/res");

    let window = match Window::new(&sdl_context, "RustyCast", WindowPos::PosCentered, WindowPos::PosCentered, WINDOW_WIDTH, WINDOW_HEIGHT, OPENGL) {
        Ok(window) => window,
        Err(err) => panic!("failed to create window: {}", err)
    };

    mouse::set_relative_mouse_mode(true);

    let mut renderer = match Renderer::from_window(window, RenderDriverIndex::Auto, ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err) => panic!("failed to create renderer: {}", err)
    };

    let mut sky = match LoadSurface::from_file(&res_path.join("sky.png")) {
        Ok(surface) => surface,
        Err(err) => panic!(format!("Failed to load png: {}", err))
    };

    let mut texture = renderer.create_texture_streaming(PixelFormatEnum::RGB24, (W as i32, H as i32)).unwrap();
    let mut drawer = renderer.drawer();
    let mut event_pump = sdl_context.event_pump();


    let mut inputs = input::InputState::new();
    let mut game = game::Game {
        sector: world::SectorIndex(0),
        pos: math::V2_ORIGIN,
        face_angle: 0.0,
        look_angle: 0.0,
        world: &world::temp(),
        show_map: false,
        t: 0.0
    };

    texture.set_blend_mode(BlendMode::None);

    // Wasted mode
    //texture.set_alpha_mod(0x22);
    //texture.set_blend_mode(BlendMode::Blend);

    'main : loop {
        let last_time = PreciseTime::now();

        for event in event_pump.poll_iter() {
            inputs.check_event(&event);
            if let Event::Quit {..} = event { break 'main; }
        }

        if inputs.has_key(input::Key::Quit) {
            break 'main;
        }

        game.step(&inputs);
        texture.with_lock(None, |buffer, _| {
            game.render(&mut sky, buffer, W, H);
        }).unwrap();

        drawer.copy(&texture, None, None);
        drawer.present();

        let delta_time = last_time.to(PreciseTime::now()).num_milliseconds();
        thread::sleep(Duration::milliseconds(15 - delta_time));
    }
}

