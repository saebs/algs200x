//! Self Grading Tool 

// MEASURE and enforce Constraints
// Runtime
// Memory used by process

// TEST
// Algorithm correctness


use std::process;
use std::process::{Command};
use std::env;
use std::time::Instant;
// use std::fs;

fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().skip(1).collect();
    //
    println!("program to grade: {}", &args[0]);
    let solution = &args[0][..];

    // trim parent folder path 'src/'
    let soln_substring = &solution[4..];
    let mut app = String::new();
    // trim solution source code extension '.rs'
    for c in soln_substring.chars() {
        if c != '.' {
            app.push(c);
        } else {
            break;
        }
    }
    println!("app name: {}", &app);
    // build solution
    let mut build = Command::new("./build.sh").arg(&solution).spawn();
                // remember to kill
    if let Ok(mut out) = build {
        // println!(" foo {:?}", out.stdout);
        out.wait(); 
    }
    let now = Instant::now();
    // run solution
    // measure solution
    // run pmap on solution, but how mmm
    // let mut pmap = Command::new("pmap")
    //             .arg();
    let elapsed = now.elapsed();

    // somewhere along the line it should read program binary to run tests on
    // or save to file
    println!("Elapsed: {:.2?}", elapsed);
    // or save to file
    println!("my Parent pid is {}", process::id());
    // println!("my Child pid is {}", procid);
    // invoke pmap on process is and print
    // pmap process::id() | tail -n 1

    Ok(())

}