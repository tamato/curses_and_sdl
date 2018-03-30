
mod context;
pub use self::context::{ConsoleContext, CursesContext, SDLContext};


mod map_gen;
pub use self::map_gen::{MapData, MapTileType};

// commands
enum Command {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
}



