package main

import (
	"fmt"
	"os"
	"strconv"
)

// 2.1 Naive Algorithm
func MaxPairwiseProductNaive() int {
	elems := os.Args[1:]
	var product int
	for _, i := range elems {

		for _, j := range elems {
			elemi, _ := strconv.Atoi(i)
			elemj, _ := strconv.Atoi(j)
			if elemi != elemj && product < elemi*elemj {
				product = elemi * elemj
			}
		}
	}
	return product
}

//  2.2 Fast algorithm

func MaxPairwiseProductFast() int {
	elstr := os.Args[1:]
	elint := make([]int, len(elstr))
	index1 := 0
	var index2 int

	for ind, i := range elstr {
		elem, _ := strconv.Atoi(i)
		elint[ind] = elem
	}

	for i, el := range elint {
		if el > elint[index1] {
			index1 = i
		}
	}
	if index1 == 0 {
		index2 = 1
	} else {
		index2 = 0
	}

	for i, el := range elint {
		if el != elint[index1] && el > elint[index2] {
			index2 = i
		}
	}
	return elint[index1] * elint[index2]
}

func main() {
	//	fmt.Printf("%v naive\n", MaxPairwiseProductNaive())
	var n int
	//	var dataSet int
	var a, b int

	fmt.Scanf("%d", &n)
	// some control shit here or constraint
	fmt.Sscanln("99\n100", &a, &b)
	fmt.Println(a, b)
	fmt.Println(n)
	fmt.Printf("%v fast\n", MaxPairwiseProductFast())

}
