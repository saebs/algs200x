package main

import (
	"fmt"
	"math"
)

/*
# python implementation of naive

result = 0

for i in range(0, n):
    for j in range(i+1, n):
        if a[i]*a[j] > result:
            result = a[i]*a[j]

print(result)

*/

// 2.1 Naive Algorithm
func MaxPairwiseProductNaive(dataset []int) int {
	var product int
	n := len(dataset)
	for i := 0; i < n; i++ {
		for j := i + 1; j < n; j++ {
			//fmt.Println(dataset[i], dataset[j])
			product = int(math.Max(float64(product), float64(dataset[i])*float64(dataset[j])))
		}
	}
	return product
}

//  2.2 Fast algorithm

func MaxPairwiseProductFast(dataset []int) int {
	index1 := 0
	var index2 int
	n := len(dataset)
	for i := 1; i < n; i++ {
		if dataset[i] > dataset[index1] {
			index1 = i
		}
	}
	if index1 == 0 {
		index2 = 1
	} else {
		index2 = 0
	}

	for i := 0; i < n; i++ {
		if i != index1 && dataset[i] > dataset[index2] {
			index2 = i
		}
	}
	fmt.Println(index1, index2)
	return dataset[index1] * dataset[index2]

}
func main() {
	//	 some control shit here or constraint
	var n int
	var m int
	fmt.Printf("n = ")
	fmt.Scanf("%d", &n)
	fmt.Printf("m = ")
	fmt.Scanf("%d", &m)
	// stress test
	StressTest(n, m)

}
