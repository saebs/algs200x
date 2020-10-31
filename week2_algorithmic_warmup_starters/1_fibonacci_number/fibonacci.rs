//! Usage:
//! ./fibonacci <n>
use std::env;

/// Naive Recursive Method
fn fib(n: i64) -> i64 {
	match n {
		0i64 => 0i64,
		1i64 => 1i64,
		_ => fib(n - 1) + fib(n -2),
    }
}

/// Optimized by Memoization Technique 
fn fib_optimised(n: usize) -> usize {
	let mut f = Vec::with_capacity(n);
	f.push(0);
	f.push(1);
	match n {
		0 => 0,
		1 => 1,
		_ => {
				for i in 2..n {
					f.push(f[i -1] + f[i -2]);
				}
				f[n-1]
		}
	}
}
fn main() {
	assert_eq!(fib(0), 0);
	assert_eq!(fib(1), 1);
	assert_eq!(fib_optimised(0), 0);
	assert_eq!(fib_optimised(1), 1);
	assert_eq!(env::args().len(), 2);
	let n: Vec<String> = env::args().skip(1).collect();
	let n = fib_optimised(n[0][..].parse::<usize>().unwrap());
	println!("{}", n);
}
