extern crate curses_and_sdl;
use curses_and_sdl::{ConsoleContext, CursesContext, SDLContext};

use std::env;
fn main() {

    let mut close_app = false;
    let mut use_curses = false;
    for arguments in env::args() {
        match arguments.as_ref() {
            "-t" | "--terminal" => use_curses = true,
            "-h" | "--help" => {
                println!("\t-t to luanch using curses");
                println!("\t-v or --version to check version");
                println!("\t-h or --help this help menu");
                close_app = true;
            }
            "-v" | "--version" => {
                println!("\tNo version info yet, still too new!");
                close_app = true;
            }
            _ => {},
        }
        // println!("arg: {:?}", arguments);
    }

    if close_app {
        return
    }

    let console_ctx: Box<ConsoleContext>;
    if use_curses {
        console_ctx = Box::new(CursesContext::new());
    } else {
        console_ctx = Box::new(SDLContext::new());
    }

    console_ctx.do_everything();
}



