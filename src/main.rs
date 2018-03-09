extern crate easycurses;

use easycurses::*;
use std::cmp::{max, min};
use std::iter::repeat;
use std::thread::sleep;
use std::time::Instant;
use std::time::Duration;

trait ConsoleContext {
    fn do_everything(&self);
}

struct CursesContext {}
impl CursesContext {
    fn new() -> Self {
        CursesContext {}
    }
}

impl ConsoleContext for CursesContext {
    fn do_everything(&self) {
        // Normal setup
        let mut easy = EasyCurses::initialize_system().unwrap();
        easy.set_cursor_visibility(CursorVisibility::Invisible);
        easy.set_echo(false);
        easy.set_keypad_enabled(true);
        easy.set_input_mode(InputMode::NonBlocking);
        easy.set_scrolling(true);

        // We need to know how wide our screen is.
        let (_, mut col_count) = easy.get_row_col_count();

        // Sadly we can't make this const since it has to unwrap and all that, but
        // ideally this could be a const. You could use lazy_static I guess if you
        // really cared, but it's not a huge deal.
        let frame_target_duration = Duration::new(1, 0)
            .checked_div(60)
            .expect("failed when rhs!=0, what?");

        // We start at an arbitrary position.
        let mut position = 5;
        loop {
            let top_of_loop = Instant::now();
            // Gather/process any pending input
            match easy.get_input() {
                Some(Input::KeyLeft) => position = max(0, position - 1),
                Some(Input::KeyRight) => position = min(col_count - 1, position + 1),
                Some(Input::KeyResize) => {
                    col_count = easy.get_row_col_count().1;
                    position = min(col_count - 1, position);
                }
                Some(Input::Character(c)) => {
                    match c {
                        'q' | 'Q' => break,
                        ch => println!("Key hit: {:?}", ch),
                    }
                }
                Some(other) => println!("Unknown: {:?}", other),
                None => (),
            }
            // Compute what we'll display.
            let output = repeat('#').take(position as usize).collect::<String>();

            // Sleep a bit if we need to. This actually sleeps a little longer than
            // just the right time because it doesn't account for the display time
            // we'll use up after the sleep happens. However, curses doesn't really
            // demand perfect animation anyway.
            let elapsed_this_frame = top_of_loop.elapsed();
            if let Some(frame_remaining) = frame_target_duration.checked_sub(elapsed_this_frame) {
                sleep(frame_remaining);
            }

            // Display
            easy.print("\n");
            easy.print(&output);
            easy.refresh();
        }
    }
}

/*************************
    SDL
*************************/
extern crate sdl2;
use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::rect::Point;

struct SDLContext {}
impl ConsoleContext for SDLContext {
    fn do_everything(&self) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("SDL2", 640, 480)
            .position_centered().build().unwrap();

        let mut canvas = window.into_canvas()
            .accelerated().build().unwrap();
        let texture_creator = canvas.texture_creator();

        canvas.set_draw_color(sdl2::pixels::Color::RGBA(0,0,0,255));

        let mut timer = sdl_context.timer().unwrap();

        let mut event_pump = sdl_context.event_pump().unwrap();

        // animation sheet and extras are available from
        // https://opengameart.org/content/a-platformer-in-the-forest
        let temp_surface = sdl2::surface::Surface::load_bmp(Path::new("assets/characters.bmp")).unwrap();
        let texture = texture_creator.create_texture_from_surface(&temp_surface).unwrap();

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
            canvas.copy_ex(&texture, Some(source_rect_0), Some(dest_rect_0), 0.0, None, false, false).unwrap();
            canvas.copy_ex(&texture, Some(source_rect_1), Some(dest_rect_1), 0.0, None, true, false).unwrap();
            canvas.copy_ex(&texture, Some(source_rect_2), Some(dest_rect_2), 0.0, None, false, false).unwrap();
            canvas.present();

            std::thread::sleep(Duration::from_millis(100));
        }
    }        
}

impl SDLContext {
    fn new() -> Self {
        SDLContext {}
    }
}

use std::env;
fn main() {

    let mut use_curses = false;
    for arguments in env::args() {
        match arguments.as_ref() {
            "-t" => use_curses = true,
            _ => {},
        }
        // println!("arg: {:?}", arguments);
    }

    let _sdl_ctx = SDLContext::new();
    let _curses_ctx = CursesContext::new();
    let console_ctx;
    if use_curses {
        console_ctx = Box::new(&_curses_ctx as &ConsoleContext);
    } else {
        console_ctx = Box::new(&_sdl_ctx as &ConsoleContext);
    }

    console_ctx.do_everything();
}