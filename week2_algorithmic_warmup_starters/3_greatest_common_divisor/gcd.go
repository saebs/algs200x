package main

import "fmt"

// type here MyFib

func gcdNaive(a, b int) int {
	current := 1
	for d := 2; d <= a && d <= b; d++ {
		if a%d == 0 && b%d == 0 {
			if d > current {
				current = d
			}
		}
	}
	return current
}

//euclid way

func gdcEuclid(a, b int) int {
	if b == 0 {
		return a
	}
	a = a % b
	return gdcEuclid(b, a)
}

// UI

func main() {
	var a, b int
	_, err := fmt.Scanf("%d %d", &a, &b)
	if err != nil {
		fmt.Println("Enter only two numbers fool")
	}
	fmt.Println(gdcEuclid(a, b))
}
