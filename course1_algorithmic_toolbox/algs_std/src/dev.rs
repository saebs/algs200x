#![macro_use]

use std::time::Instant;
use std::mem;

/// Measures the Algorithm Running Time in seconds
#[macro_export]
macro_rules! running_time {
    ($sol_n:ident($($n:expr),*)) => {
       let moment = Instant::now(); 
       $sol_n($($n),*)
       let done = moment.elapsed();
       println!("Running Time: {:.3?}", &done.as_secs_f64());
       done
    };
    // consider options with limits or constraints
}


#[macro_export]
macro_rules! soln_eq {
    ($naive_soln:ident, $fast_soln:ident, $($arg:expr)+) => {
        let result1 = $naive_soln($($arg)+);
        let result2 = $fast_soln($($arg)+);
        if !(&result1 == &result2) {
            panic!("Wrong Answer!:  `{:?}`  `{:?}` ", *result1, *result2);
        } else {
            println!("Ok");
        }
    };
}