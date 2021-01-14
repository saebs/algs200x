#![macro_use]

use std::io::{Error, ErrorKind};
fn ohsh_t() -> std::io::Error {
    Error::new(ErrorKind::Other, "oh no!")
}

pub fn foo() {
    println!("{}", crate::randy::bool());
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
// TODO: lets create a macro yeh?
pub fn parse_input(problem_i: &InputFormat) -> std::io::Result<InputFormat> {
    let mut buff = String::new();
    // options
    match problem_i {
        InputFormat::IntegerN(_) => {
            ::std::io::stdin().read_line(&mut buff)?;
            let mut line1 = buff.split_whitespace();
            let n: i64 = line1.next().unwrap().parse::<i64>().unwrap();
            Ok(InputFormat::IntegerN(n))
        }
        InputFormat::IntegersNandM(_) => {
            ::std::io::stdin().read_line(&mut buff)?;
            let mut line1 = buff.split_whitespace();
            let n: i64 = line1.next().unwrap().parse::<i64>().unwrap();
            let m: i64 = line1.next().unwrap().parse::<i64>().unwrap();
            Ok(InputFormat::IntegersNandM(vec![n, m]))
        }
        InputFormat::SeqOfnNIntegers(_) => {
            ::std::io::stdin().read_line(&mut buff)?;
            let seq_of_n_integers: Vec<i64> = buff
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect();
            Ok(InputFormat::SeqOfnNIntegers(seq_of_n_integers))
        }
        InputFormat::Nlines(_) => {
            // let mut _nlines = ::std::io::stdin().lock();
            // parse all line to i64 and into collection of vectors
            let all = vec![];
            Ok(InputFormat::Nlines(all))
        }
        _ => Err(ohsh_t()),
    }
}
