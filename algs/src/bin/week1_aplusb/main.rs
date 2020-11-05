use std::env;
use std::io::{self, Read};
use std::fs;
use std::process;

fn aplusb_naive(a: i32, b: i32) -> i32 {
    a + b
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 1 {
        println!("not enough arguments");
        process::exit(0)
    }
    println!("{}", &args[0][..]); 
    Ok(())

}