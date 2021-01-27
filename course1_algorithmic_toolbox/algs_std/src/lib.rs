//! The course "Standard" library customised for this course.
//! Provides useful components for common Programming challenges as well as external tools
#![crate_type="lib"]
#![crate_name="algs_std"]
#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]
#![allow(unused_imports)]
use std::io::{Error, ErrorKind};
use std::io;
use std::io::prelude::*;
use std::iter::repeat_with;
// pub extern crate fastrand;
pub use fastrand;



/// thumbs up
pub const PASSMOJI: char =  '\u{1F44D}'; 
/// thumbs down
pub const FAILMOJI: char =  '\u{1F44E}';



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
/// args: limit, solution call
#[macro_export]
macro_rules! running_time {
    ($limit:expr,$sol_n:ident($($n:expr),*)) => {
       let moment = std::time::Instant::now(); 
       $sol_n($($n),*);
       let time = moment.elapsed();
       println!("\nRunning Time: {:.3?}\n", &time.as_secs_f64());
       if $limit as f64 >= time.as_secs_f64() {
           println!("\n {} Pass\n", $crate::PASSMOJI);
           assert!(true);
       }
       else {
           println!("\n {} Fail\n", $crate::FAILMOJI);
           assert!(false);
       }
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
            println!("{} Wrong Answer!:  {:?}  {:?}", $crate::FAILMOJI,$x, $y );
            false 
        } else {
            println!("{} Ok", $crate::PASSMOJI);
            true
        }
    };
}

#[macro_export]
/// Test generator with constraints n and m
macro_rules! gen_seq_of_n  {
    ($n:expr,$m:expr) => {{ 
        let r = $crate::fastrand::i64(2..$n);
        let seq: Vec<i64> = std::iter::repeat_with(|| $crate::fastrand::i64(2..$n)).take(r as usize).collect();
        seq
    }};
}



/// takes two identifiers of algorithms and a seed value
#[macro_export]
macro_rules! stress_test {
    ($model:ident, $main:ident, $n:expr, $m:expr) => { // zero more tokens

        loop {
            let numbers = $crate::gen_seq_of_n!($n, $m);
            println!("{:?}", &numbers);
            let r1 = $model(&numbers);
            let r2 = $main(&numbers);
            let state = test_eq!(r1, r2);
            if  !state {
                break;
                
            }
        }
        assert!(false);
    };
}


    