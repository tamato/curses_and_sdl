use std::path::Path;
use std::time::Duration;

extern crate sdl2;
use self::sdl2::event::Event;
use self::sdl2::keyboard::Keycode;
use self::sdl2::rect::Rect;
use self::sdl2::rect::Point;

use context::ConsoleContext;

pub struct SDLContext {}
impl ConsoleContext for SDLContext {
    fn do_everything(&self) {
        let sdl_context = sdl2::init().expect("Failed to create sdl_context");
        let video_subsystem = sdl_context.video().expect("Failed to get video_subsystem");

        let window = video_subsystem.window("SDL2", 640, 480)
            .position_centered().build().expect("Failed to acquire window from video_subsystem");

        let mut canvas = window.into_canvas()
            .accelerated().build().expect("Failed to aqcuire canvas");
        let texture_creator = canvas.texture_creator();

        canvas.set_draw_color(sdl2::pixels::Color::RGBA(0,0,0,255));

        let mut timer = sdl_context.timer().expect("Failed to get timer");

        let mut event_pump = sdl_context.event_pump().expect("Failed to get event_pump");

        // animation sheet and extras are available from
        // https://opengameart.org/content/a-platformer-in-the-forest
        let temp_surface = sdl2::surface::Surface::load_bmp(Path::new("assets/characters.bmp")).expect("Failed to load characters.bmp");
        let texture = texture_creator.create_texture_from_surface(&temp_surface).expect("Failed to get texture from texture_creator");

        let frames_per_anim = 4;
        let sprite_tile_size = (32,32);

        // Baby - walk animation
        let mut source_rect_0 = Rect::new(0, 0, sprite_tile_size.0, sprite_tile_size.0);
        let mut dest_rect_0 = Rect::new(0, 0, sprite_tile_size.0*4, sprite_tile_size.0*4);
        dest_rect_0.center_on(Point::new(-64,120));

        // King - walk animation
        let mut source_rect_1 = Rect::new(0, 32, sprite_tile_size.0, sprite_tile_size.0);
        let mut dest_rect_1 = Rect::new(0, 32, sprite_tile_size.0*4, sprite_tile_size.0*4);
        dest_rect_1.center_on(Point::new(0,240));

        // Soldier - walk animation
        let mut source_rect_2 = Rect::new(0, 64, sprite_tile_size.0, sprite_tile_size.0);
        let mut dest_rect_2 = Rect::new(0, 64, sprite_tile_size.0*4, sprite_tile_size.0*4);
        dest_rect_2.center_on(Point::new(440,360));

        let mut running = true;
        while running {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                        running = false;
                    },
                    _ => {}
                }
            }

            let ticks = timer.ticks() as i32;

            // set the current frame for time
            source_rect_0.set_x(32 * ((ticks / 100) % frames_per_anim));
            dest_rect_0.set_x(1 * ((ticks / 14) % 768) - 128);

            source_rect_1.set_x(32 * ((ticks / 100) % frames_per_anim));
            dest_rect_1.set_x((1 * ((ticks / 12) % 768) - 672) * -1);

            source_rect_2.set_x(32 * ((ticks / 100) % frames_per_anim));
            dest_rect_2.set_x(1 * ((ticks / 10) % 768) - 128);

            canvas.clear();
            // copy the frame to the canvas
            canvas.copy_ex(&texture, Some(source_rect_0), Some(dest_rect_0), 0.0, None, false, false).expect("Failed to copy source 0");
            canvas.copy_ex(&texture, Some(source_rect_1), Some(dest_rect_1), 0.0, None, true, false).expect("Failed to copy source 1");
            canvas.copy_ex(&texture, Some(source_rect_2), Some(dest_rect_2), 0.0, None, false, false).expect("Failed to copy source 2");
            canvas.present();

            ::std::thread::sleep(Duration::from_millis(100));
        }
    }        
}

impl SDLContext {
    pub fn new() -> Self {
        SDLContext {}
    }
}




