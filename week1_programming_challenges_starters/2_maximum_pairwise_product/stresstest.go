package main

/* A stress test

1. your solution or algorithm implementation.
2. an alternative, trivial and slow, but correct implementation of an algorithm for same problem.
3. a random test generator
4. an infinite loop in which a new test is generated and fed to both implementations to compare results.
*/

import (
	"fmt"
	"math/rand"
)

func StressTest(n, m int) {
	datasetA := make([]int, n)
	for {
		if n < 2 || n > m { // enforcing n constraints
			fmt.Println("n must be >= 2 and < m")
			break
		}
		for i := 0; i < n; i++ {
			datasetA[i] = rand.Intn(m + 1) // to include m since the built in function returns number upto m -1
		}
		fmt.Println(datasetA)
		result1 := MaxPairwiseProductNaive(datasetA)
		result2 := MaxPairwiseProductFast(datasetA)
		if result1 == result2 {
			fmt.Println("OK")
		} else {
			fmt.Println("Wrong Answer", result1, result2)
			return
		}
	}
}
