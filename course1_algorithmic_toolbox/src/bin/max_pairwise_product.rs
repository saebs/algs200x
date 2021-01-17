use algs_std::io::*;
/*
MaxPairwiseProductNaive(A[1 : : :n]):
product <- 0
for i from 1 to n:
    for j from i + 1 to n:
        product <- max(product;A[i] A[j])
return product
*/

// Fails when number not sorted.
fn max_pairwise_naive(numbers: &Vec<i64>) -> i64 {
    let n = numbers.len();
    let mut product: i64 = 0;
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

fn max_pairwise_product_fast(numbers: &Vec<i64>) -> i64 {
    let n = numbers.len();
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

    let n = read_integer_n();
    let numbers = read_integer_seq(n as usize).unwrap(); 


    let fin = max_pairwise_product_fast(&numbers);
    println!("{}", fin);

    // algs_std::test_eq!(max_pairwise_naive, max_pairwise_product_fast, &numbers);

    Ok(())
}
