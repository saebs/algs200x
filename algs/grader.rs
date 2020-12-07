//! Self Grading Tool 

// MEASURE and enforce Constraints
// Runtime
// Memory used by process

// TEST
// Algorithm correctness


use std::process;
use std::process::Command;
use std::env;
use std::time::Instant;
use std::fs;



fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().skip(1).collect();
    //
    println!("program to grade: {}", &args[0]);
    let prog = &args[0].clone();
    let prog = prog.as_str();
    // let prog = fs::canonicalize(prog)?;
    let app = Command::new("sh")
            .current_dir("build.sh")
            .arg(prog)
            .spawn()
            .expect("mm kulokuthancileyo");
    let now = Instant::now();
    
    app.stdout;
    // somwhere along the line it should read program binary to run tests on
    
    let elapsed = now.elapsed();
    // or save to file
    println!("Elapsed: {:.2?}", elapsed);
    // or save to file
    println!("my pid is {}", process::id());

    Ok(())

}