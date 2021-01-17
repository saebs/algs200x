#![macro_use]

use std::io::{Error, ErrorKind};

pub fn foo() {
    println!("{}", crate::randy::bool());
}

#[derive(Debug)]
pub enum InputFormat {
    IntegerN(i64),
    IntegersNandM(Vec<i64>),
    SeqOfnNIntegers(Vec<i64>),
    Nlines(Vec<Vec<i64>>),
    IntegerNandSeqOfNIntegers(i64, Vec<i64>),
    NonNegativeIntegersNandM(Vec<u64>),
}

pub fn read_integer_n() -> i64 {
    let input = io::stdin().lock().lines();
    loop {
            for line in io::stdin().lock().lines() {
                let input = line.expect("Failed to read line");
                match input.trim().parse::<i64>() {
                    Ok(n) => n,
                    Err(e) => println!("Failed to read integer: {}", e),
                }
        }
    }
}

pub fn read_integer_seq(max_elements: i64) -> Vec<i64> {
    // collect into vec i64 upto max_elements
    unimplemented!();

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