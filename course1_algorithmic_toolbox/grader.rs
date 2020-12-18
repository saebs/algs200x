//! Self Grading Tool

// MEASURE and enforce Constraints
// Runtime
// Memory used by process

// TEST
// Algorithm correctness

use std::env;
use std::process::{Command, Stdio};
use std::time::Instant;
// use std::fs;
// use std::str::from_utf8;
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    // println!("program to grade: {}", &args[0]);
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

    println!("solution name: {}\n", &app);
    // build solution
    Command::new("./build.sh")
        .arg(&solution)
        .spawn()
        .unwrap()
        .wait()
        .expect("ma 1");

    // TODO Modularise The Benchmarking Code to re-use when test suites and PASS, FAIL logic implemented

    // Solution Running Time Bechmark
    //***********************************
    let now = Instant::now();
    let mut solution_child = Command::new("bin/./".to_owned() + &app).spawn().unwrap();
    let solution_process_id = solution_child.id().to_string();
    let elapsed = now.elapsed();

    // Memory Used by process Benchmark
    // ***************************************************

    // #!/bin/bash
    // pmap 6860 | tail -n 1 | awk 'END {print $NF}'

    let pmap = Command::new("pmap")
        .arg(&solution_process_id)
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to start");
    let pmap_out = pmap.stdout.expect("Failed bafo");

    let tail = Command::new("tail")
        .arg("-n 1")
        .stdin(Stdio::from(pmap_out))
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start tail part");
    let tail_out = tail.stdout.expect("Kwehlule futhi");

    let awk = Command::new("awk")
        .arg("END {print $NF}")
        .stdin(Stdio::from(tail_out))
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start  awk part");
    let awk_out = awk.wait_with_output().expect("awk part fail");
    let memory_used: String = match std::str::from_utf8(awk_out.stdout.as_slice()) {
        Ok(ram) => ram.to_string(),
        Err(_) => "Something went wrong".to_owned(),
    };

    // until solution process done
    match solution_child.wait() {
        Ok(_) => println!("-- Ok --"),
        _ => println!("something went wrong"),
    }

    // TODO or save to file
    println!("\ntime: {:.2?}", elapsed);
    println!("memory: {}", &memory_used);
    Ok(())
}
