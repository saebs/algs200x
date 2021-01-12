extern crate stress;
/*

MaxPairwiseProductNaive(A[1 : : :n]):
product <- 0
for i from 1 to n:
    for j from i + 1 to n:
        product <- max(product;A[i] A[j])
return product

*/

// Fails when number not sorted.
fn max_pairwise_naive(n: u64, numbers: &Vec<u64>) -> u64 {
    let mut product: u64 = 0;
    for i in 1..n {
        for j in i + 1..n {
            product = cmp::max(product, numbers[i as usize] * numbers[j as usize]);
        }
    }
    product
}

/*


MaxPairwiseProductFast(A[1...n]):
index1 <-  1
    for i from 2 to n:
        if A[i] > A[index1]:
            index1 <- i

if index1 = 1:
    index2 <- 2
else:
    index2 <- 1

for i from 1 to n:
    if A[i] != A[index1] and A[i] > A[index2]:
        index2 <- i
return A[index1] * A[index2]
*/

fn max_pairwise_product_fast(n: u64, numbers: &Vec<u64>) -> u64 {
    let mut index1 = 0usize;
    for i in 1..n {
        if numbers[i as usize] > numbers[index1] {
            index1 = i as usize;
        }
    }

    let mut index2 = 0usize;
    if index1 == 0usize {
        index2 = 1;
    } 

    // println("two fer {}", index2);

    for i in 0..n {
        if numbers[i as usize] != numbers[index1] && numbers[i as usize] > numbers[index2] {
            index2 = i as usize;
        }
    }
    numbers[index1] * numbers[index2]
}

use std::cmp;
fn main() -> std::io::Result<()> {
    let mut buff = String::new();
    ::std::io::stdin().read_line(&mut buff)?;
    let mut line1 = buff.split_whitespace();
    // collect into vec of u64s
    let n: u64 = line1.next().unwrap().parse::<u64>().unwrap();
    // eprintln!("{:?}", n);
    let mut line2 = String::new();
    ::std::io::stdin().read_line(&mut line2)?;
    let numbers: Vec<u64> = line2
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect();
    // eprintln!("{:?}", numbers);
    // println!("{}", max_pairwise_product_fast(n, &numbers));
    stress::test!(2u64, 1000u64, max_pairwise_naive, max_pairwise_product_fast);

    Ok(())
}