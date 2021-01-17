//! Custom Input and Output utilities
//! Standardises the Input formats specified by the Programming Challenges
//! use them to help you focus on the Problem sets rather that input implementation
#![macro_use]
#![allow(unused_imports)]
use std::io::{Error, ErrorKind};
use std::io;
use std::io::prelude::*;


/// Read an Integer from Standard input / cli
pub fn read_integer_n() ->  i64 {
    let stdin = io::stdin();
    loop {
            for line in stdin.lock().lines() {
                let input = line.expect("Failed to read line");
                match input.trim().parse::<i64>() {
                    Ok(n) => return n,
                    Err(e) => println!("Failed to read number {}", e), 
                }
        }
    }
}

/// Read A sequence of `n` integers from Standard Input /cli
pub fn read_integer_seq(max_elements: usize) -> std::io::Result<Vec<i64>> {
    // collect into vec i64 upto max_elements
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let seq: Vec<i64> = buffer.split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .take(max_elements)
        .collect();
       Ok(seq) 

}
