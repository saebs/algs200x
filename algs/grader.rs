//! supposed to run various requirements tests
// Runtime
// Memory used
// Algorithm correctness
use std::fmt::*;

// compile with optimisations
// rustc -O grader.rs
fn main() {
    //
    use std::time::Instant;
    let now = Instant::now();
    
    for i in 0..1000000 {
        println!("foo");
    }
    
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}