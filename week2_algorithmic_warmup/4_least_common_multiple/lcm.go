package main

import "fmt"

func lcmNaive(a, b int) int {
	for l := 1; l <= a*b; l++ {
		if l%a == 0 && l%b == 0 {
			return l
		}
	}
	return a * b
}

type fibo struct {
	a, b int
}

// euclid func
func euc(a, b int) int {
	if b == 0 {
		return a
	}
	a = a % b
	return euc(b, a)
}

// LcmF

func (f fibo) lcmFast() int {
	euc(f.a, f.b)
	return f.a / euc(f.a, f.b) * f.b
}

//UI

func main() {
	var a, b int
	_, err := fmt.Scanf("%d %d", &a, &b)
	if err != nil {
		fmt.Printf("%v", err)
	}
	m := fibo{a, b}
	fmt.Println(m.lcmFast())
	// fmt.Println(lcmNaive(a, b))
}
