package main

import (
	"fmt"
	"math"
	"os"
	"strconv"
)

// 2.1 Naive Algorithm
func MaxPairwiseProductNaive() float64 {
	elems := os.Args[1:]
	var product float64
	for _, i := range elems {

		for _, j := range elems {
			elemi, _ := strconv.ParseFloat(i, 64)
			elemj, _ := strconv.ParseFloat(j, 64)
			if elemi != elemj {
				product = math.Max(product, elemi*elemj)
			}
		}
	}
	return product
}

func main() {
	fmt.Println("maximum pairwise go implementation")
	fmt.Printf("%v", MaxPairwiseProductNaive())

}
