package main

import (
	"fmt"
	"math/rand"
)

func stressTest() {
	for {
		n := rand.Intn(45)
		result1 := fibNaive(n)
		fib := newFibonacci(n)
		result2 := fib.nth()
		if result1.Cmp(result2) == 0 { // remember big int comparison!!!
			fmt.Println("OK")
		} else {
			fmt.Println("Wrong Answer", result1, result2)
			break
		}
	}
}
