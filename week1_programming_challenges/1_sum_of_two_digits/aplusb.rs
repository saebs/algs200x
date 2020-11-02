use std::env;
// use std::fs;

fn aplusb(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let inputs: Vec<String> = env::args().skip(1).collect();
    // todo: parse input to integers 
    let x = 1;
    let v = 1;
    assert_eq!(2i32, aplusb(x, v))
}