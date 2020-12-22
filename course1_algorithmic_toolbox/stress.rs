
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

pub fn test(n: u64, m: u64) {
    unimplemented!("TO stress you")
}