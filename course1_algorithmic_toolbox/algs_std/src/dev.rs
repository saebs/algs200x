#![macro_use]

#[macro_export]
macro_rules! test {
    // The pattern for a single `eval`
    ($n:expr, $model_soln:ident, $main_soln:ident ) => {{
        {
            let input1: u64 = $n; // Force types to be integers
            let result1 = $model_soln(&vec![input1, 2u64]);
            let result2 = $main_soln(&vec![input1, 2u64]);
            if result1 == result2 {
                println!("OK");
            } else {
                println!("Wrong answer: {}, {}", $model_soln(), $main_soln());
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
                println!("Wrong answer: {}, {}", result1, result2);
                // break;
            }
        }
    }};
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
