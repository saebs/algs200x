//_rdrand64_step
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

#[macro_export]
macro_rules! test {
    ($x:expr, $model_soln:ident, $main_soln:ident) => {
        if $model_soln($x) == $main_soln($x) {
            println!("OK");
        } else {
            println!("Wrong answer: {}, {}", $model_soln($x), $main_soln($x));
            std::process::exit();
        }
    };
    ($model_soln:ident($a:expr, $b:expr), $main_soln:ident($c:expr, $d:expr)) => {
        if $model_soln($a, $b) == $main_soln($c, $d) {
            println!("OK");
        } else {
            println!("Wrong answer: {}, {}", $model_soln($a, $b), $main_soln($c, $d));
            std::process::exit();
        }
    };
}