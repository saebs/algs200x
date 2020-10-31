//! Usage:
//! ./fibonacci <n>
use std::env;

/// Naive Recursive Method

/// Optimized by Memoization Technique 
fn fib_last_optimised(n: usize) -> usize {
	let mut f = Vec::with_capacity(n);
	f.push(0);
	f.push(1);
	match n {
		0 => 0,
		1 => 1,
		_ => {
				for i in 2..n+1 {
					f.push((f[i -1] + f[i -2])%10);
				}
				f[n]
		}
	}
}
fn main() {
	assert_eq!(env::args().len(), 2);
	assert_eq!(fib_last_optimised(0), 0);
	assert_eq!(fib_last_optimised(1), 1);
	let n: Vec<String> = env::args().skip(1).collect();
	let n = fib_last_optimised(n[0][..].parse::<usize>().unwrap());
	println!("{}", n);
}
