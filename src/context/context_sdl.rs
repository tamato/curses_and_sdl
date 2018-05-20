use std::path::Path;
use std::time::Duration;

extern crate sdl2;
use self::sdl2::event::Event;
use self::sdl2::keyboard::Keycode;
use self::sdl2::rect::{Rect, Point};

use self::sdl2::image::{LoadTexture, INIT_PNG};

use context::ConsoleContext;
use map_gen::{MapTileType, tcod_tutorial};
use {World, CommandMoveTo, Command, ResourceCollection, CommandCollection};

use std::cmp::{max, min};

const TILE_WIDTH: i32 = 12;
const TILE_HEIGHT: i32 = 22;

const MAP_WIDTH: usize = 53;
const MAP_HEIGHT: usize = 12;

const GAME_WINDOW_WIDTH: u32 =  1024;
const GAME_WINDOW_HEIGHT: u32 = 800;

pub struct SDLContext {}
impl ConsoleContext for SDLContext {
    fn do_everything(&self) {

        let mut resources = ResourceCollection::new();
        let mut commands = CommandCollection::new();

        let sdl_context = sdl2::init().expect("Failed to create sdl_context");
        let video_subsystem = sdl_context.video().expect("Failed to get video_subsystem");

        let window = video_subsystem.window("SDL2", GAME_WINDOW_WIDTH, GAME_WINDOW_HEIGHT)
            .position_centered()
            .build()
            .expect("Failed to acquire window from video_subsystem");

        let mut canvas = window.into_canvas()
            .accelerated()
            .build()
            .expect("Failed to aqcuire canvas");

        let texture_creator = canvas.texture_creator();

        canvas.set_draw_color(sdl2::pixels::Color::RGBA(0,0,0,255));

        // let timer = sdl_context.timer().expect("Failed to get timer");

        let mut event_pump = sdl_context.event_pump().expect("Failed to get event_pump");

        let _image_context = sdl2::image::init(INIT_PNG).expect("failed to get image context");

        let mut texture22 = texture_creator.load_texture(Path::new("assets/font22.png"))
            .expect("Failed to load 22x22");
        let tile_width = TILE_WIDTH as u32;
        let tile_height = TILE_HEIGHT as u32;

        let map = tcod_tutorial::map_generation(MAP_WIDTH, MAP_HEIGHT);
        let tiles = TILE::new();

        let player_rect = Rect::new(TILE_WIDTH * 0, TILE_HEIGHT * 4, TILE_WIDTH as u32, TILE_HEIGHT as u32);
        let mut player_x = 5 as i32;
        let mut player_y = 5 as i32;

        let mut world = World::new();
        // world.add_char(0, player_x, player_y);
        resources.add(0, Point::new(player_x, player_y));

        let mut running = true;
        while running {
            let mut got_input = false;
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                        running = false;
                    },
                    Event::KeyDown {keycode: Some(key), ..} => {
                        match key {
                            Keycode::Q => running = false,
                            Keycode::E | Keycode::K => player_y = { got_input=true; max(1, player_y - 1)},
                            Keycode::D | Keycode::J => player_y = { got_input=true; min( (MAP_HEIGHT - 2) as i32, player_y + 1)},
                            Keycode::F | Keycode::L => player_x = { got_input=true; min(MAP_WIDTH as i32 - 2, player_x + 1)},
                            Keycode::S | Keycode::H => player_x = { got_input=true; max(1, player_x - 1)},
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }

            if got_input == true {
                let cmd = Box::new( CommandMoveTo::new(0, player_x, player_y) );
                commands.add(cmd);
            }
            
            // handle commands
            let cmds = commands.cmds;
            for cmd in cmds {
                cmd.handle(&mut resources);
            }
            commands = CommandCollection::new();

            canvas.clear();

            texture22.set_color_mod(255, 255, 255);

            // Display
            map.render( |w, _, data| {
                // data is a 1 dim vector, but we treat it as 2d because it make far more sense
                for (idx, tile) in data.iter().enumerate() {
                    let width = *w as usize;
                    let y = idx / width;
                    let x = idx % width;
                    let dest = Rect::new( 
                        (x * tile_width as usize) as i32, 
                        (y * tile_height as usize) as i32, 
                        tile_width, 
                        tile_height);

                    canvas.copy_ex(&texture22, Some(tiles[ *tile ]), Some(dest), 0.0, None, false, false)
                        .expect("Failed to set map");
                }
            });

            let player_coord = resources.positions[&0];
            let player_dest = Rect::new(player_coord.x * TILE_WIDTH, player_coord.y * TILE_HEIGHT, TILE_WIDTH as u32, TILE_HEIGHT as u32);

            // blank out where the character is
            let blank_rect = Rect::new(TILE_WIDTH *11, TILE_HEIGHT * 13, TILE_WIDTH as u32, TILE_HEIGHT as u32);
            texture22.set_color_mod(0,0,0);
            canvas.copy_ex(&texture22, Some(blank_rect), Some(player_dest), 0.0, None, false, false).expect("Failed to load player");

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

