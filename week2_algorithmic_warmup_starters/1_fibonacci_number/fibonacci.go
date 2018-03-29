package main

import (
	"fmt"
	"math"
	"math/big"
)

func calcFibFast(n int) int {

	part1 := 1/2 + (math.Sqrt(5) / 2)
	fmt.Println(part1)
	part2 := 1/2 - (math.Sqrt(5) / 2)
	fmt.Println(part2)
	fn := 1 / math.Sqrt(5) * (math.Pow(part1, float64(n)) - math.Pow(part2, float64(n)))
	return int(fn)
}

// Binets arithmetic formula
func fibBinets(n big.Int) big.Int {
	if n <= 1 {
		return n
	}
	//phi := (1 + big.Sqrt(5))* 0.5
	// TO DO
	return n.Exp(math.Sqrt(5) + 0.5)

}

func main() {
	var n big.Int
	fmt.Scanf("%d", &n)
	fmt.Printf("%d", fibBinets(n))
}
