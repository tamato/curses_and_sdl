extern crate curses_and_sdl;
use curses_and_sdl::{ConsoleContext, CursesContext, SDLContext};

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