package main

import (
	"fmt"
	"math"
)

func calcFibFast(n int) int {

	part1 := 1/2 + (math.Sqrt(5) / 2)
	fmt.Println(part1)
	part2 := 1/2 - (math.Sqrt(5) / 2)
	fmt.Println(part2)
	fn := 1 / math.Sqrt(5) * (math.Pow(part1, float64(n)) - math.Pow(part2, float64(n)))
	return int(fn)
}

func main() {
	var n int
	fmt.Scanf("%d", &n)
	fmt.Printf("%d", calcFibFast(n))
}
