// https://stackoverflow.com/questions/26212397/references-to-traits-in-structs
pub trait ConsoleContext {
    fn do_everything(&self);
}

pub mod context_curses;
pub use self::context_curses::CursesContext;

pub mod context_sdl;
pub use self::context_sdl::SDLContext;
