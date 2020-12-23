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
extern crate stress;

fn sum_of_two_digits(a: u64, b: u64) -> u64 {
    a + b 
}

fn main() -> std::io::Result<()> {
    stress::test!(sum_of_two_digits(1u64, 3u64), sum_of_two_digits(1u64, 1u64));    
    let mut buff = String::new();
    ::std::io::stdin().read_line(&mut buff)?;
    let mut words = buff.split_whitespace();

    let a: u64 = words.next().unwrap().parse().unwrap();
    let b: u64 = words.next().unwrap().parse().unwrap();

    println!("{}", sum_of_two_digits(a, b));
    Ok(())
}