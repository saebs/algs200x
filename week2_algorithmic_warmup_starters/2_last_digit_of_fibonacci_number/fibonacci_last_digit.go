package main

import "fmt"

func getFibonacciLastDigitNaive(n int) int {
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
	return current % 10
}

func getFibonacciLastDigitFast(n int) int {
	if n <= 1 {
		return n
	}
	var previous int
	current := 1

	for i := 0; i < n-1; i++ {
		tmpPrevious := previous
		previous = current
		current = (tmpPrevious + current) % 10
	}
	return current
}

func main() {
	var n int
	fmt.Scanf("%d", &n)
	fmt.Println(getFibonacciLastDigitFast(n))
}
