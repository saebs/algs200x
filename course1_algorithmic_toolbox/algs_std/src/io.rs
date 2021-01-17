#![macro_use]

use std::io::{Error, ErrorKind};
use std::io;
use std::io::prelude::*;

pub fn foo() {
    println!("{}", crate::randy::bool());
}

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


// generic input handling
// TODO: lets create a macro yeh?
    /*ptions
            ::std::io::stdin().read_line(&mut buff)?;
            let mut line1 = buff.split_whitespace();
            let n: i64 = line1.next().unwrap().parse::<i64>().unwrap();
            let m: i64 = line1.next().unwrap().parse::<i64>().unwrap();
            Ok(InputFormat::IntegersNandM(vec![n, m]))
        InputFormat::SeqOfnNIntegers(_) => {
            ::std::io::stdin().read_line(&mut buff)?;
            let seq_of_n_integers: Vec<i64> = buff
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect();
            Ok(InputFormat::SeqOfnNIntegers(seq_of_n_integers))
        }

        */