#![crate_type="lib"]
#![crate_name="algs_std"]
#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]
//! The Standard library customised for this course.
//! Provides useful utilies and Data types from testing, Input parsing and extending build tool


mod fastrand;
#[macro_use]
pub mod io {
    use ::fastrand;
    use std::io::{Error, ErrorKind};
    fn ohsh_t() -> std::io::Error {
        Error::new(ErrorKind::Other, "oh no!")
    }

    pub fn foo() {
        println!("{}", fastrand::bool());
    }

    #[derive(Debug)]
    pub enum InputFormat {
        IntegerN(i64),
        IntegersNandM(Vec<i64>),
        SeqOfnNIntegers(Vec<i64>), 
        Nlines(Vec<Vec<i64>>),
        IntegerNandSeqOfNIntegers(i64, Vec<i64>),
        NonNegativeIntegersNandM(Vec<u64>),
    }
    // generic input handling
    pub fn parse_input(problem_i: &InputFormat) -> std::io::Result<InputFormat>  {
        let mut buff = String::new();
        // options
        match problem_i {
            InputFormat::IntegerN(_)=> {
            ::std::io::stdin().read_line(&mut buff)?;
            let mut line1 = buff.split_whitespace();
            let n: i64 = line1.next().unwrap().parse::<i64>().unwrap();
            Ok(InputFormat::IntegerN(n))
            },
            InputFormat::IntegersNandM(_) => {
            ::std::io::stdin().read_line(&mut buff)?;
            let mut line1 = buff.split_whitespace();
            let n: i64 = line1.next().unwrap().parse::<i64>().unwrap();
            let m: i64 = line1.next().unwrap().parse::<i64>().unwrap();
            Ok(InputFormat::IntegersNandM(vec![n, m]))
            },
            InputFormat::SeqOfnNIntegers(_) => {
            ::std::io::stdin().read_line(&mut buff)?;
            let seq_of_n_integers: Vec<i64> = buff 
            .split_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect();
            Ok(InputFormat::SeqOfnNIntegers(seq_of_n_integers))
            }, 
            InputFormat::Nlines(_) => {
                // let mut _nlines = ::std::io::stdin().lock();
                // parse all line to i64 and into collection of vectors
                let all = vec![];
                Ok(InputFormat::Nlines(all))
            },
            _ => {
                Err(ohsh_t())
            }

        }
    }


}

#[macro_use]
pub mod stress {
    use fastrand;
    #[macro_export]
    macro_rules! test {
        // The pattern for a single `eval`
        ($n:expr, $model_soln:ident, $main_soln:ident ) => {{
            {
                let input1: u64  = $n; // Force types to be integers
                let result1 = $model_soln(&vec![input1, 2u64]);
                let result2 = $main_soln(&vec![input1, 2u64]);
                if result1 == result2 {
                    println!("OK");
                } else {
                    println!(
                        "Wrong answer: {}, {}",
                        $model_soln(),
                        $main_soln()
                    );
                    // break;
                }
            }
        }};
    
        ($n:expr, $m:expr, $model_soln:ident, $main_soln:ident ) => {{
            {
                let input1: u64 = $n; // Force types to be integers
                let input2 = vec![$m, 2u64];
                // TODO 
                // generate random integer between 2 and n
                // generate and allocate array with random from 0 to m
                let result1 = $model_soln(input1, &input2);
                let result2 = $main_soln(input1, &input2);
                if result1 == result2 {
                    println!("OK");
                } else {
                    println!(
                        "Wrong answer: {}, {}",
                        result1,
                        result2 
                    );
                    // break;
                }
            }
        }}; 
    }
}

/*


StressTest(N;M):
    while true:
        n random integer between 2 and N
        allocate array A[1 : : :n]
        for i from 1 to n:
            A[i] random integer between 0 and M
        print(A[1 : : :n])
            result1  MaxPairwiseProductNaive(A)
            result2  MaxPairwiseProductFast(A)
        if result1 = result2:
            print(“OK”)
        else:
            print(“Wrong answer: ”, result1, result2)
            return
*/