package main

import (
	"fmt"
)

func getFibonacciHugeNaive(n, m int) int {
	if n <= 1 {
		return n
	}
	var previous int
	current := 1
	for i := 0; i < n-1; i++ {
		tmpPrevious := previous
		previous = current
		current = tmpPrevious + current
	}
	return current % m
}
func getFibonacciHugeFast(n, m int) int {
	if n <= 1 {
		return n
	}
	var previous int
	current := 1
	var fibs []int
	fibs = append(fibs, 0, 1)
	var rem int
	for i := 0; i < n-1; i++ {
		tmpPrevious := previous
		previous = current
		current = tmpPrevious + current
		fibs = append(fibs, current)
		// fmt.Println(fibs)
		// Pissano period
		if previous%m == 0 && current%m == 1 {
			rem = n % (i + 1) // by length of period i+1
		}
	}
	return fibs[rem] % m
}

func main() {
	var n, m int
	fmt.Scanf("%d %d", &n, &m)
	fmt.Println(getFibonacciHugeFast(n, m))
}
