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
// Constraints. 0  a;b  9.
// Sample.
// Input:
// 9 7
// Output:
// 16
// Time limits (sec.):
// C C++ Java Python Haskell JavaScript Scala
// 1 1 1.5 5 2 5 3
// Memory limit. 512 Mb.
extern crate algs_std;
use algs_std::io::*;

fn sum_of_two_digits(a: i64, b: i64) -> i64 {
    a + b 
}
 
fn main() -> std::io::Result<()> {
    let a = read_integer_n();
    let b = read_integer_n();

    println!("{}", sum_of_two_digits(a, b));
    Ok(())
}