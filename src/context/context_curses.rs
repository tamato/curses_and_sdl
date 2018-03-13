
extern crate easycurses;
use self::easycurses::*;

use std::cmp::{max, min};
use std::thread::sleep;
use std::time::Instant;
use std::time::Duration;

use map_gen::{MapTileType, tcod_tutorial};

use context::ConsoleContext;

pub struct CursesContext {}
impl CursesContext {
    pub fn new() -> Self {
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
        let (row_count, col_count) = easy.get_row_col_count();

        // Sadly we can't make this const since it has to unwrap and all that, but
        // ideally this could be a const. You could use lazy_static I guess if you
        // really cared, but it's not a huge deal.
        let frame_target_duration = Duration::new(1, 0)
            .checked_div(60)
            .expect("failed when rhs!=0, what?");


        let map = tcod_tutorial::map_generation((col_count-2) as usize, (row_count-2) as usize);
        let tiles = ASCII::new();

        // We start at an arbitrary position.
        let mut pos_x = 5;
        let mut pos_y = 5;
        loop {
            let top_of_loop = Instant::now();
            // Gather/process any pending input
            match easy.get_input() {
                Some(Input::KeyLeft) => pos_x = max(1, pos_x - 1),
                Some(Input::KeyRight) => pos_x = min(col_count - 3, pos_x + 1),
                Some(Input::KeyUp) => pos_y = max(1, pos_y - 1),
                Some(Input::KeyDown) => pos_y = min(row_count - 3, pos_y + 1),
                Some(Input::Character(c)) => {
                    match c {
                        'q' | 'Q' => break,
                        'e' => pos_y = max(3, pos_y - 1),
                        'd' => pos_y = min(row_count - 3, pos_y + 1), 
                        'f' => pos_x = min(col_count - 3, pos_x + 1),
                        's' => pos_x = max(3, pos_x - 1),
                        'r' => {
                            pos_x = min(col_count - 3, pos_x + 1); 
                            pos_y = max(3, pos_y - 1);
                        }
                        'w' => {
                            pos_x = max(3, pos_x - 1);
                            pos_y = max(3, pos_y - 1);
                        }
                        'c' => {
                            pos_x = max(3, pos_x - 1);
                            pos_y = min(row_count - 3, pos_y + 1); 
                        }
                        'v' => {
                            pos_y = min(row_count - 3, pos_y + 1); 
                            pos_x = min(col_count - 3, pos_x + 1);
                        }
                        ch => println!("Key hit: {:?}", ch),
                    }
                }
                Some(other) => println!("Unknown: {:?}", other),
                None => (),
            }

            // Sleep a bit if we need to. This actually sleeps a little longer than
            // just the right time because it doesn't account for the display time
            // we'll use up after the sleep happens. However, curses doesn't really
            // demand perfect animation anyway.
            let elapsed_this_frame = top_of_loop.elapsed();
            if let Some(frame_remaining) = frame_target_duration.checked_sub(elapsed_this_frame) {
                sleep(frame_remaining);
            }

            // Display
            map.render( |w, _, data| {
                // data is a 1 dim vector, but we treat it as 2d because it make far more sense
                for (idx, tile) in data.iter().enumerate() {
                    let col_count = *w as usize;
                    let map_row = idx / col_count;
                    let map_col = idx % col_count;

                    easy.move_rc(map_row as i32, map_col as i32);
                    easy.print_char(tiles[ *tile ]);
                }
            });

            easy.move_rc(pos_y, pos_x);
            easy.print_char('@');
            easy.refresh();
        }
    }
}

struct ASCII {
    floor: char,
    wall: char,
    chasm: char,
    water: char,
}

impl ASCII {
    fn new() -> Self {
        ASCII {
            floor: '.',
            wall: '#',
            chasm: 'X',
            water: '~',
        }
    }
}

use std::ops::Index;
impl Index<MapTileType> for ASCII {
    type Output = char;
    fn index(&self, index: MapTileType) -> &Self::Output {
        match index {
            MapTileType::Floor => &self.floor,
            MapTileType::Wall => &self.wall,
            MapTileType::Chasm => &self.chasm,
            MapTileType::Water => &self.water,
        }
    }
}















