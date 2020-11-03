use std::env;
use std::io::{self, Read};
// use std::fs;

fn aplusb(a: i32, b: i32) -> i32 {
    a + b
}

pub fn run() -> io::Result<()> {
    let inputs: Vec<String> = env::args().skip(1).collect();
    // todo: parse input to integers 
    let x = 1;
    let v = 1;
    assert_eq!(2i32, aplusb(x, v));
    let mut buffy = String::from(&inputs[0]);
    match io::stdin().read_line(&mut buffy) {
        Ok(buffy) => println!("Hi {}", buffy),
        Err(err) => println!("dakwaz: {}", err),
    }
    Ok(())
}