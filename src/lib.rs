
mod context;
pub use self::context::{ConsoleContext, CursesContext, SDLContext};

mod map_gen;
pub use self::map_gen::{MapData, MapTileType};

// https://users.rust-lang.org/t/solved-is-it-possible-to-clone-a-boxed-trait-object/1714/6
// https://users.rust-lang.org/t/understanding-trait-composition-and-box/8844/6
impl Clone for Box<Command> {
    fn clone(&self) -> Self {
        self.cmd_clone()
    }
}

use std::collections::HashMap;
pub struct World {
    entities: Vec<u32>,
}

impl World {
    fn new () -> Self {
        World {
            entities: Vec::new(),
        }
    }
}


extern crate sdl2;
use sdl2::rect::Point;
pub struct ResourceCollection {
    positions: HashMap<u32, Point>,
    dummy: HashMap<u32, i32>,
}

impl ResourceCollection {
    fn new() -> Self {
        ResourceCollection {
            positions: HashMap::new(),
            dummy: HashMap::new(),
        }
    }

    fn add(&mut self, ent: u32, data: Point) {
        self.positions.insert(ent, data);
    }

    fn add2(&mut self, ent: u32, data: i32) {
        self.dummy.insert(ent, data);
    }
}

pub struct CommandCollection {
    cmd_move: Vec<CommandMoveTo>,
}

impl CommandCollection {
    fn new() -> Self {
        CommandCollection {
            cmd_move: Vec::new(),
        }
    }

    fn add(&mut self, data: CommandMoveTo) {
        self.cmd_move.push(data);
    }
}

pub trait Command {
    fn handle(&self, &mut ResourceCollection);
    // fn handle(&self, &mut)
    fn cmd_clone(&self) -> Box<Command>;
}

// the move command does not do any error checking
// that is done by a different command
#[derive(Clone, Copy)]
pub struct CommandMoveTo {
    // priority: u32,

    object: u32,
    destination: Point,
}

impl CommandMoveTo {
    pub fn new(who: u32, x: i32, y: i32) -> Self {
        CommandMoveTo {
            object: who,
            destination: Point::new(x,y),
        }
    }
}

impl Command for CommandMoveTo {
    fn handle(&self, res: &mut ResourceCollection) {
        *res.positions.get_mut(&0).unwrap() = self.destination;
    }
    fn cmd_clone(&self) -> Box<Command> {
        Box::new( (*self).clone())
    }
}

