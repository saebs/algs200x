//! Self Grading Tool 

// MEASURE and enforce Constraints
// Runtime
// Memory used by process

// TEST
// Algorithm correctness


use std::process;
use std::process::{Command, Stdio};
use std::env;
use std::time::Instant;
// use std::fs;
use std::str::from_utf8;
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
        out.wait(); 
    }
    let now = Instant::now();
    // solution spawned here
    // record / capture process id
    // run many tests soon as well
    let elapsed = now.elapsed();

// ***************************************************
// IMPLENTING THE BASH SCRIPT  
// to extract the memory used by a specified process id e.g.
//  
// $ pmap 6860 | tail -n 1 | awk 'END {print $NF}'
//
//****************************************************

    // run solution
    // run pmap on solution, but how mmm
    let proc_id = process::id().to_string();
    // pmap ID ...
    let pmap = Command::new("pmap")
        .arg(proc_id)
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to start");
    
    let pmap_out = pmap.stdout.expect("Failed bafo");

    // ... | tail -n 1
    let tail = Command::new("tail")
        .arg("-n 1")
        .stdin(Stdio::from(pmap_out))
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start tail part");
    
    let tail_out = tail.stdout.expect("kwehlule");
// ... | awk 'END {print $NF}'
    let awk = Command::new("awk")
        .arg("END {print $NF}")
        .stdin(Stdio::from(tail_out))
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start  awk part");

    let awk_out = awk.wait_with_output().expect("awk part fail");
    let mut mem_used = String::new(); 
    match std::str::from_utf8(awk_out.stdout.as_slice()) {
        Ok(memo) => mem_used = memo.to_string(),
        Err(_) => (),
    }

    
    // somewhere along the line it should read program binary to run tests on
    // TODO or save to file
    println!("time: {:.2?}", elapsed);
    println!("memory: {}", &mem_used);
    println!("Parent pid is {}", process::id());
    Ok(())

}