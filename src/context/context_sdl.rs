use std::path::Path;
use std::time::Duration;

extern crate sdl2;
use self::sdl2::event::Event;
use self::sdl2::keyboard::Keycode;
use self::sdl2::rect::Rect;
use self::sdl2::rect::Point;

use self::sdl2::image::{LoadTexture, INIT_PNG};

use context::ConsoleContext;
use map_gen::{MapTileType, tcod_tutorial};

use std::cmp::{max, min};

const TILE_WIDTH: i32 = 12;
const TILE_HEIGHT: i32 = 22;

const WINDOW_WIDTH: u32 = 53 * TILE_WIDTH as u32;
const WINDOW_HEIGHT: u32 = 12 * TILE_HEIGHT as u32;

pub struct SDLContext {}
impl ConsoleContext for SDLContext {
    fn do_everything(&self) {
        let sdl_context = sdl2::init().expect("Failed to create sdl_context");
        let video_subsystem = sdl_context.video().expect("Failed to get video_subsystem");

        let window = video_subsystem.window("SDL2", WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .build()
            .expect("Failed to acquire window from video_subsystem");

        let mut canvas = window.into_canvas()
            .accelerated()
            .build()
            .expect("Failed to aqcuire canvas");

        let texture_creator = canvas.texture_creator();

        canvas.set_draw_color(sdl2::pixels::Color::RGBA(0,0,0,255));

        let mut timer = sdl_context.timer().expect("Failed to get timer");

        let mut event_pump = sdl_context.event_pump().expect("Failed to get event_pump");

        let _image_context = sdl2::image::init(INIT_PNG).expect("failed to get image context");

        let mut texture22 = texture_creator.load_texture(Path::new("assets/font22.png"))
            .expect("Failed to load 22x22");
        let tile_width = TILE_WIDTH as u32;
        let tile_height = TILE_HEIGHT as u32;

        let win_width = ((WINDOW_WIDTH) / tile_width) as usize;
        let win_height = ((WINDOW_HEIGHT) / tile_height) as usize;
        let map = tcod_tutorial::map_generation(win_width, win_height);
        let tiles = TILE::new();

        let player_rect = Rect::new(TILE_WIDTH * 0, TILE_HEIGHT * 4, TILE_WIDTH as u32, TILE_HEIGHT as u32);
        let mut player_x = 5 as i32;
        let mut player_y = 5 as i32;

        let mut running = true;
        while running {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                        running = false;
                    },
                    Event::KeyDown {keycode: Some(key), ..} => {
                        match key {
                            Keycode::Q => running = false,
                            Keycode::E | Keycode::K => player_y = max(1, player_y - 1),
                            Keycode::D | Keycode::J => player_y = min( (win_height - 2) as i32, player_y + 1),
                            Keycode::F | Keycode::L => player_x = min(win_width as i32 - 2, player_x + 1),
                            Keycode::S | Keycode::H => player_x = max(1, player_x - 1),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }

            let ticks = timer.ticks() as i32;
            canvas.clear();

            texture22.set_color_mod(255, 50, 50);

            // Display
            map.render( |w, _, data| {
                // data is a 1 dim vector, but we treat it as 2d because it make far more sense
                for (idx, tile) in data.iter().enumerate() {
                    let width = *w as usize;
                    let y = idx / width;
                    let x = idx % width;
                    let dest = Rect::new( (x * tile_width as usize) as i32, (y * tile_width as usize) as i32, tile_width, tile_height);

                    canvas.copy_ex(&texture22, Some(tiles[ *tile ]), Some(dest), 0.0, None, false, false).expect("Failed to set map");
                }
            });

            let player_dest = Rect::new(player_x * TILE_WIDTH, player_y * TILE_HEIGHT, TILE_WIDTH as u32, TILE_HEIGHT as u32);

            texture22.set_color_mod(255, 255, 255);
            texture22.set_alpha_mod(255);
            canvas.copy_ex(&texture22, Some(player_rect), Some(player_dest), 0.0, None, false, false).expect("Failed to load player");

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

struct TILE {
    floor: Rect,
    wall: Rect,
    chasm: Rect,
    water: Rect,
}

impl TILE {
    fn new() -> Self {
        TILE {
            floor: Rect::new( TILE_WIDTH *  7, TILE_HEIGHT *  0, TILE_WIDTH as u32, TILE_HEIGHT as u32),
            wall:  Rect::new( TILE_WIDTH * 11, TILE_HEIGHT * 13, TILE_WIDTH as u32, TILE_HEIGHT as u32),
            chasm: Rect::new( TILE_WIDTH *  0, TILE_HEIGHT *  0, TILE_WIDTH as u32, TILE_HEIGHT as u32),
            water: Rect::new( TILE_WIDTH *  7, TILE_HEIGHT * 15, TILE_WIDTH as u32, TILE_HEIGHT as u32),
        }
    }
}

use std::ops::Index;
impl Index<MapTileType> for TILE {
    type Output = Rect;
    fn index(&self, index: MapTileType) -> &Self::Output {
        match index {
            MapTileType::Floor => &self.floor,
            MapTileType::Wall => &self.wall,
            MapTileType::Chasm => &self.chasm,
            MapTileType::Water => &self.water,
        }
    }
}


