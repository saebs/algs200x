//! The course "Standard" library customised for this course.
//! - Provides reusable components for common programming challenges tasks,such as getting input
//! - helpers tools for debugging / enforcing constraints  or semantics
//! - Test generators ..
#![crate_type="lib"]
#![crate_name="algs_std"]
#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]
#![allow(unused_imports)]
use std::io::{Error, ErrorKind};
use std::io;
use std::io::prelude::*;
use std::iter::repeat_with;
pub use fastrand;

// thumbs up
// pub const PASSMOJI: char =  '\u{1F44D}'; 
// thumbs down
// pub const FAILMOJI: char =  '\u{1F44E}';

/// Ok button
pub const PASSMOJI: char = '\u{1F197}';
/// Red X
pub const FAILMOJI: char =  '\u{274C}';

/// Input Format:
/// From Standard input / cli
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

/// Input format:
/// Sequence of  upto `n` integers from Standard Input
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

/// Benchmark tool:
/// Set Algorithm expected time limit
#[macro_export]
macro_rules! running_time {
    ($limit:expr,$sol_n:ident($($n:expr),*)) => {
       let moment = std::time::Instant::now(); 
       $sol_n($($n),*);
       let time = moment.elapsed();
       let time = time.as_secs_f64();
       println!("\n\n{}\nRunning Time: {:.2?}s", std::stringify!($sol_n) ,&time);
       if $limit as f64 >= time {
           println!("{} Pass", $crate::PASSMOJI);
           time
        //    assert!(true);
       }
       else {
           println!("{} Fail", $crate::FAILMOJI);
           time
        //    assert!(false);
       }
    };
}



/// Compare correct but naive solution against fast(er) solution 
#[macro_export]
macro_rules! test_eq {
    ($x:expr, $y:expr) => {
        if !($x == $y) {
            println!("{} Wrong Answer!:  {:?} <> {:?}\n", $crate::FAILMOJI,$x, $y );
            false 
        } else {
            println!("{}\n", $crate::PASSMOJI);
            true
        }
    };
}

#[macro_export]
/// Generic Test generator 
/// One Number, Sequences of Numbers
macro_rules! test_gen  {
    ($n:expr,$m:expr) => {{ 
        let r = $crate::fastrand::i64(2..$n);
        let seq: Vec<i64> = std::iter::repeat_with(|| $crate::fastrand::i64(2..$n)).take(r as usize).collect();
        seq
    }};
    ($n:expr) => {{ 
        let n = $crate::fastrand::i64(2..$n);
        n
    }};
}


/// Rigourous Randomized equality tests 
/// Comparing a Model solution against Fast(er) implementation,  
#[macro_export]
macro_rules! stress_test {
    // Sequence Of Numbers
    ($model:ident, $main:ident, $n:expr, $m:expr) => { 

        loop {
            let numbers = $crate::test_gen!($n, $m);
            println!("{:?}", &numbers);
            let r1 = $model(&numbers);
            let r2 = $main(&numbers);
            let state = test_eq!(r1, r2);
            if  !state {
                break;
                
            }
        }
        // assert!(false);
    };
    // One Number
    ($model:ident, $main:ident, $n:expr) => { 

        loop {
            let number = $crate::test_gen!($n);
            println!("{:?}", number);
            let r1 = $model(number);
            let r2 = $main(number);
            let state = test_eq!(r1, r2);
            if  !state {
                break;
                
            }
        }
        // assert!(false);
    };
}

// Todo , Memory Tests ?????



    