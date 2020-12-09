//! Self Grading Tool 

// MEASURE and enforce Constraints
// Runtime
// Memory used by process

// TEST
// Algorithm correctness


use std::process;
use std::process::{Command,Stdio};
use std::env;
use std::time::Instant;
// use std::fs;



fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().skip(1).collect();
    //
    println!("program to grade: {}", &args[0]);
    let prog = &args[0].clone();
    let _prog = prog.as_str();
    // let prog = fs::canonicalize(prog)?;
    let now = Instant::now();
    let procid = format!("{}",process::id());
    let _piper =  " | tail -n 1".to_owned();
    let app = Command::new("pmap")
        .arg(&procid)
        .stdin(Stdio::piped())
        .spawn()
        .expect("mm kulokuthancileyo")
        .wait();
    let elapsed = now.elapsed();

    // somewhere along the line it should read program binary to run tests on
    // or save to file
    println!("Elapsed: {:.2?}", elapsed);
    // or save to file
    println!("my pid is {}", process::id());
    // invoke pmap on process is and print
    // pmap process::id() | tail -n 1

    Ok(())

}