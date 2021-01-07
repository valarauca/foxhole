#![allow(unused_must_use)]
#![allow(dead_code)]
#![allow(unused_imports)]

mod cli;
mod internals;

fn main() {
    let rc = match cli::run() {
        Ok(()) => 0i32,
        Err(e) => {
            eprint!("{}", e);
            1i32
        }
    };
    std::process::exit(rc);
}
