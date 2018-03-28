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
func fibonacciSumLastDigit(n int) int {
	if n <= 2 {
		return n
	}
	var previous int
	current := 1
	var fibs []int
	fibs = append(fibs, 0, 1)
	for i := 0; i < n-1; i++ {
		tmpPrevious := previous
		previous = current
		current = tmpPrevious + current
		fibs = append(fibs, current)
	}
	return (fibonacciSumLastDigit(n-1) + fibs[n]) % 10
}

func main() {
	var n int
	fmt.Scanf("%d", &n)
	fmt.Println(fibonacciSumLastDigit(n))
}
