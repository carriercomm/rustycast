extern crate sdl;

use self::sdl::video::{SurfaceFlag, VideoFlag};
use self::sdl::event::{Event};

use std::num::Float;

static WIDTH:  usize = 320;
static HEIGHT: usize = 240;


fn get_color(x: usize, y: usize, t: usize) -> (u8, u8, u8) {
    let fx = x as f32;
    let fy = y as f32;
    let dist = Float::sqrt(fx*fx + fy*fy) as u8;

    (dist, 3*(y as u8)-0xFF-10*(t as u8), 7*(t as u8))
}


pub fn real_main() {
    sdl::init(&[sdl::InitFlag::Video]);
    sdl::wm::set_caption("RustyCast", "RustyCast");

    let screen = match sdl::video::set_video_mode(WIDTH as isize, HEIGHT as isize, 24,
                                                  &[SurfaceFlag::HWSurface],
                                                  &[VideoFlag::DoubleBuf]) {
        Ok(screen) => screen,
        Err(err) => panic!("failed to set video mode: {}", err)
    };

    let mut t = 0;

    'main : loop {
        'event : loop {
            match sdl::event::poll_event() {
                Event::Quit => break 'main,
                Event::None => break 'event,
                _ => {}
            }
        }

        screen.with_lock(|pixels| {
            for x in 0..WIDTH {
                for y in 0..HEIGHT {
                    let (r, g, b) = get_color(x, y, t);
                    pixels[3*(WIDTH*y+x) + 0] = b;
                    pixels[3*(WIDTH*y+x) + 1] = g;
                    pixels[3*(WIDTH*y+x) + 2] = r;
                }
            }
            true
        });

        t = t + 1;

        screen.flip();
    }

    sdl::quit();
}
