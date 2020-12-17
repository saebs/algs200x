/*

MaxPairwiseProductNaive(A[1 : : :n]):
product <- 0
for i from 1 to n:
    for j from i + 1 to n:
        product <- max(product;A[i] A[j])
return product

*/

// Fails when number not sorted. 
fn max_pairwise_naive(n: u32, numbers: & Vec<u32>)
-> u32
{
    let mut product: u32 = 0;
    for i in 1..n {
        for j in i+1..n {
            product = cmp::max(product , numbers[i as usize] * numbers[j as usize]);
        }
    }
    product
}
 use std::cmp;
fn main() -> std::io::Result<()> {

    let info = r##" 
    Sample 1.
    Input: 
    3
    1 2 3
    Output:
    6
    ---------------------------
    Sample 2.
    Input:
    10
    7 5 14 2 8 8 10 1 2 3
    Output:
    140
    "##;
    println!("{}", info);

    let mut buff = String::new();
    ::std::io::stdin().read_line(&mut buff)?;
    let mut  line1 = buff.split_whitespace();
    // collect into vec of u32s
    let n: u32 = line1.next().unwrap().parse::<u32>().unwrap();
    // eprintln!("{:?}", n);
    let mut line2 = String::new();
    ::std::io::stdin().read_line(&mut line2)?;
    let numbers: Vec<u32> = line2
                    .split_whitespace()
                    .map(| n | n.parse::<u32>().unwrap())
                    .collect();
    // eprintln!("{:?}", numbers);
    match n {
        3u32 => assert_eq!(6, max_pairwise_naive(n, &numbers)),
        10u32 => assert_eq!(140, max_pairwise_naive(n, &numbers)),
        _ => println!("Output: {}",max_pairwise_naive(n, &numbers)),

    } 
    Ok(())
}