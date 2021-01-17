//! Development tools 
//! Use these as frameworks for your test suits and debugging activities
#![macro_use]


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

/// Tests equality of pair of algorithms provided with one or more common input.
/// correct but naive solution vs fast implementation
/// component ideally used for stress testing
#[macro_export]
macro_rules! test_eq {
    ($naive_soln:ident, $fast_soln:ident, $($arg:expr),+) => {
        let result1 = $naive_soln($($arg),+);
        let result2 = $fast_soln($($arg),+);
        if !(result1 == result2) {
            println!("Wrong Answer!:  `{:?}`  `{:?}` ", result1, result2);
            std::process::exit();
        } else {
            println!("Ok");
        }
    };
    ($naive_soln:ident, $fast_soln:ident, $($arg:expr),+) => {
        let result1 = $naive_soln($($arg),+);
        let result2 = $fast_soln($($arg),+);
        if !(result1 == result2) {
            println!("Wrong Answer!:  `{:?}`  `{:?}` ", result1, result2);
            std::process::exit();
        } else {
            println!("Ok");
        }
    };
}