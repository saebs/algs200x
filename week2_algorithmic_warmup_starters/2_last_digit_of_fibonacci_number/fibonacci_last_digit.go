package main

import (
	"fmt"
	"math/big"
)

func fibLastDigitNaive(n int) *big.Int {
	if n <= 1 {
		return big.NewInt(int64(n))
	}
	var lastDigit big.Int
	previous := big.NewInt(0) // also the first fibonacci number
	current := big.NewInt(1)

	for i := 0; i < n-1; i++ {
		current, previous = previous, current
		current.Add(previous, current)
	}
	return lastDigit.Mod(current, big.NewInt(int64(10)))
}

func fibLastDigitFast(n int) *big.Int {
	if n <= 1 {
		return big.NewInt(int64(n))
	}
	var lastDigit big.Int
	previous := big.NewInt(0) // also the first fibonacci number
	current := big.NewInt(1)

	for i := 0; i < n-1; i++ {
		current, previous = previous, current
		lastDigit.Mod(current.Add(previous, current), big.NewInt(int64(10)))

	}
	return &lastDigit
}

func main() {
	var n int
	fmt.Scanf("%d", &n)
	if 0 <= n && n <= 100000000 {
		fmt.Println(fibLastDigitNaive(n))
		fmt.Println(fibLastDigitFast(n))
	}
}
