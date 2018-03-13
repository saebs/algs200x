package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

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
	//debuggin fmt.Println(index1, index2)
	return dataset[index1] * dataset[index2]
	// write data set
	//func writedata(

}

// reads first line and second line of input the produces data set
func getDataset(n int) []int {
	nRawSeq := bufio.NewReader(os.Stdin) // next line input: A1,...,An
	elems, _, _ := nRawSeq.ReadLine()
	rawElems := strings.Split(string(elems), " ") //
	cleanElems := make([]int, n)

	for i, num := range rawElems {
		cleanElems[i], _ = strconv.Atoi(num)
	}
	return cleanElems
}

func main() {
	//  Constraints 2<= n <= 2*10exp5 and A1,...An <= 2*10exp5
	var n int
	fmt.Scanf("%d", &n) // first line input: n
	sequence := getDataset(n)
	fmt.Println(MaxPairwiseProductFast(sequence))

	// STRESS TEST
	/*
		var int m
		fmt.Scanf("%d", &m
		StressTest(n, m)

	*/
}
