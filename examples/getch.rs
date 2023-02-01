extern crate getch_rs;
use getch_rs::{Getch, Key};

fn main() {
    let g = Getch::new();

    println!("press `q` to exit");

    loop {
        match g.getch() {
            Ok(Key::Char('q')) => break,
            Ok(key) => println!("{:?}", key),
            Err(e) => println!("{}", e),
        }
    }
}
