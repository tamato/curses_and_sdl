pub trait ConsoleContext {
    fn do_everything(&self);

    // fn 
}

mod context_curses;
pub use self::context_curses::CursesContext;

mod context_sdl;
pub use self::context_sdl::SDLContext;
