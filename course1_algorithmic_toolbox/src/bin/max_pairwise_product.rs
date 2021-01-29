use algs_std::*;
/*
MaxPairwiseProductNaive(A[1 : : :n]):
product <- 0
for i from 1 to n:
    for j from i + 1 to n:
        product <- max(product;A[i] A[j])
return product
*/

// Fails when number not sorted.
#[allow(dead_code)]
fn max_pairwise_naive(numbers: &Vec<i64>) -> i64 {
    let n = numbers.len();
    let mut product: i64 = 0;
    for i in 0..n {
        for j in (i + 1)..n {
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
    algs_std::test_eq!("ahe", "hezvo");
    let fin = max_pairwise_product_fast(&numbers);
    println!("{}", fin);
    Ok(())
}



#[cfg(test)]
mod tests {
    use algs_std::{self, running_time};
    use crate::{max_pairwise_naive, max_pairwise_product_fast, test_eq};


    #[test]
    fn max_pairwise() {
        // let status = algs_std::test_eq!(max_pairwise_naive(&vec![1i64,2i64]), max_pairwise_product_fast( &vec![1i64, 2i64]));
        // Use this with custom test_eq! macro
        algs_std::stress_test!(max_pairwise_naive, max_pairwise_product_fast, 10i64, 100000i64);
    }

    #[test]
    fn max_pairwise_time () {
       let d = algs_std::test_gen!(6, 1000); 
       running_time!(1f64, max_pairwise_naive(&d));
    }

    #[test]
    fn mem_used() {
    //    let d = algs_std::test_gen!(10, 10000000000000000000000000000000000007); 
    //     memory_used!(max_pairwise_naive(&d));
        let mut foo = algs_std::Memory::new();
            foo.begin();
            let _itsho = vec![100000000; 10000000];
            foo.end();
        print!("Memory used: {}", foo.usage() );
        
    }

}
