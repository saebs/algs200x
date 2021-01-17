//! The course "Standard" library customised for this course.
//! Provides useful components for common Programming challenges as well as external tools
#![crate_type="lib"]
#![crate_name="algs_std"]
#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]
#![allow(unused_imports)]
use std::io::{Error, ErrorKind};
use std::io;
use std::io::prelude::*;


/// Input Formats:
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
    let mut buffer = String::with_capacity(max_elements);
    io::stdin().read_line(&mut buffer)?;
    let seq: Vec<i64> = buffer.split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .take(max_elements)
        .collect();
    assert!(seq.len() == max_elements, "below the required number of integers");
    Ok(seq) 

}

///Development tool:
/// Use these as frameworks for your test suits and debugging activities
/// Measures the Algorithm Running Time in seconds
#[macro_export]
macro_rules! running_time {
    ($sol_n:ident($($n:expr),*)) => {
       let moment = std::time::Instant::now(); 
       $sol_n($($n),*)
       let time = moment.elapsed();
       println!("Running Time: {:.3?}", &time.as_secs_f64());
       time
    };
    // consider options with limits or constraints
}

/// Development tool: Check equality
/// Tests equality of two expressions 
/// returns a boolean. specialy design to be used in stress test loop 
/// correct but naive solution vs fast implementation
/// component ideally used for stress testing
#[macro_export]
macro_rules! test_eq {
    ($x:expr, $y:expr) => {
        if !($x == $y) {
            println!("Wrong Answer!:  {:?}  {:?}", $x, $y);
            false 
        } else {
            println!("Ok");
            true
        }
    };
}

/// takes two identifiers of algorithms and a seed value
#[macro_export]
macro_rules! stress_test {
    ($model:ident, $main:ident, $s:expr) => { // zero more tokens
        loop {
            let seed: i64 = $s;
            println!("{}", seed);
            let mut numbers: Vec<i64> = Vec::new();
            for i in 0..seed {
            numbers.push( seed + i); // replace this shit 
            }
            println!("{:?}", &numbers);
            let r1 = $model(seed, seed);
            let r2 = $main(seed, seed);
            let correct = test_eq!(r1, r2);
            if !correct {
            break ; 
            }
        }
    };
}


    
/*loop {
        let nrand = 0;
        println!("{}", nrand);
        let mut numbers: Vec<i64> = Vec::new();
        for i in 0..10 {
        numbers.push( nrand + i); // replace this shit 
        }
        println!("{:?}", &numbers);
        let correct = test_eq!(sum_of_two_digits(1,2), zombili( 1,2 ));
        if !correct {
        break ; 
        }
    }
    */