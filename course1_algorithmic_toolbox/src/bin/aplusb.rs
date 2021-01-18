//! Week 1 
//! APlusB
// Sum of Two Digits Problem
// Compute the sum of two single digit numbers.
// Input: Two single digit numbers.
// Output: The sum of these numbers.
// 2 + 3 = 5
// We start from this ridiculously simple problem to show you the
// pipeline of reading the problem statement, designing an algorithm, implementing
// it, testing and debugging your program, and submitting it to
// the grading system.
// Input format. Integers a and b on the same line (separated by a space).
// Output format. The sum of a and b.
// Constraints. 0 =< a;b =< 9.
// Sample.
// Input:
// 9 7
// Output:
// 16
// Time limits (sec.):
// C C++ Java Python Haskell JavaScript Scala
// 1 1 1.5 5 2 5 3
// Memory limit. 512 Mb.

use algs_std::*;

fn sum_of_two_digits(a: i64, b: i64) -> i64 {
    assert!(((0 <= a && b <= 9) && (0 <= b && a <= 9)), "oustide of constraints" );
    a + b 
}

fn zombili(a: i64, b: i64) -> i64 {
    assert!(((0 <= a && b <= 9) && (0 <= b && a <= 9)), "oustide of constraints" );
    a + b 
}

fn main() -> std::io::Result<()> {
    let _a: i64 = read_integer_n();
    let _b: i64 = read_integer_n();

    // println!("{}", sum_of_two_digits(a, b));

    // loop {
    //     let nrand = 0;
    //     println!("{}", nrand);
    //     let mut numbers: Vec<i64> = Vec::new();
    //     for i in 0..10 {
    //     numbers.push( nrand + i); // replace this shit 
    //     }
    //     println!("{:?}", &numbers);
    //     let correct = test_eq!(sum_of_two_digits(1,2), zombili( 1,2 ));
    //     if !correct {
    //     break ; 
    //     }
    // }
    stress_test!(sum_of_two_digits, zombili, 3);

    Ok(())
}