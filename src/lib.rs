
mod context;
pub use self::context::{ConsoleContext, CursesContext, SDLContext};

mod map_gen;
pub use self::map_gen::{MapData, MapTileType};

pub trait Command {
    fn handle(&self, &mut World);
    fn cmd_clone(&self) -> Box<Command>;
}

// the move command does not do any error checking
// that is done by a different command
#[derive(Clone, Copy)]
pub struct CommandMoveTo {
    // priority: u32,

    object: u32,
    destination_x: i32,
    destination_y: i32,
}

impl CommandMoveTo {
    pub fn new(who: u32, x: i32, y: i32) -> Self {
        CommandMoveTo {
            object: who,
            destination_x: x,
            destination_y: y,
        }
    }
}
 impl Command for CommandMoveTo {fn handle(&self, world: &mut World) {
        if self.object == world.player_id {
            *(world.pos_x.get_mut(&self.object).unwrap()) = self.destination_x;
            *(world.pos_y.get_mut(&self.object).unwrap()) = self.destination_y;
        }
    }
    fn cmd_clone(&self) -> Box<Command> {
        Box::new( (*self).clone())
    }
}

// https://users.rust-lang.org/t/solved-is-it-possible-to-clone-a-boxed-trait-object/1714/6
// https://users.rust-lang.org/t/understanding-trait-composition-and-box/8844/6
impl Clone for Box<Command> {
    fn clone(&self) -> Self {
        self.cmd_clone()
    }
}

use std::collections::HashMap;
pub struct World {
    player_id: u32,

    pos_x: HashMap<u32, i32>,
    pos_y: HashMap<u32, i32>,

    commands: Box<Command>,
}

impl World {
    fn new () -> Self {
        World {
            player_id: 0,
            pos_x: HashMap::new(),
            pos_y: HashMap::new(),
            commands: Box::new(CommandMoveTo::new(0,0,0)),
        }
    }
    fn handle_commands(&mut self) {
        let cmd = self.commands.clone();
        cmd.handle(self);

        self.commands = Box::new(CommandMoveTo::new(0,0,0));
    }

    fn add_char(&mut self, id: u32, x: i32, y: i32) {
        self.player_id = id;
        self.pos_x.entry(id).or_insert(x);
        self.pos_y.entry(id).or_insert(y);
    }

    fn add_command(&mut self, cmd: Box<Command>) {
        self.commands = cmd;
    }

    fn player_coord(&mut self) -> (i32, i32) {
        (*(self.pos_x.entry(self.player_id).or_insert(0)), *(self.pos_y.entry(self.player_id).or_insert(0)))
    }
}
