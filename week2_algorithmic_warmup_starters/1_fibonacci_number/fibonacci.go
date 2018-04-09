package main

import (
	// "math"
	"fmt"
	"math/big"
)

//Fibonacci Naive
func fibNaive(n int) *big.Int {
	if n <= 1 {
		return big.NewInt(int64(n))
	}
	previous := big.NewInt(0) // also the first fibonacci number
	current := big.NewInt(1)

	for i := 0; i < n-1; i++ {
		current, previous = previous, current
		current.Add(previous, current)
	}
	return current
}

// fibonacci item

type fibonacci struct {
	n int
	/* fibonacci 2x2 and Q matrices
	   [a b]
	   [c d]
	*/
	fa, fb, fc, fd big.Int
	qa, qb, qc, qd big.Int
}

func newFibonacci(n int) fibonacci {
	// this is our constructor
	f := fibonacci{}
	f.n = n
	f.fa.SetInt64(int64(1))
	f.fb.SetInt64(int64(1))
	f.fc.SetInt64(int64(1))
	f.fd.SetInt64(int64(0))
	f.qa.SetInt64(int64(1))
	f.qb.SetInt64(int64(1))
	f.qc.SetInt64(int64(1))
	f.qd.SetInt64(int64(0))
	// fmt.Println(&f.qa, &f.qb)
	// fmt.Println(&f.qc, &f.qd)
	return f
}

// Matrix exponentiation method
func (fib *fibonacci) nth() *big.Int {
	if fib.n <= 1 {
		return big.NewInt(int64(fib.n))
	}
	fib.power()
	return &fib.fa
}

// Helper function calcs F[][] to power n, enters value in F[][]

func (fib *fibonacci) power() {
	// multiply matrxi by itself n-1 times
	for i := 2; i <= fib.n-1; i++ {
		fib.multiply()
	}
	// fmt.Println(F)
}

// 2x2 matrix multiplier
func (fib *fibonacci) multiply() {
	var a, b, c, d, x, y big.Int
	// adding dot produc sums
	// filling fibonacci matrix
	a.Add(x.Mul(&fib.fa, &fib.qa), y.Mul(&fib.fb, &fib.qc))
	b.Add(x.Mul(&fib.fa, &fib.qb), y.Mul(&fib.fb, &fib.qd))
	c.Add(x.Mul(&fib.fc, &fib.qa), y.Mul(&fib.fd, &fib.qc))
	d.Add(x.Mul(&fib.fc, &fib.qb), y.Mul(&fib.fd, &fib.qd))
	fib.fa = a
	fib.fb = b
	fib.fc = c
	fib.fd = d
	// fmt.Println(&fib.fa, &fib.fb)
	// fmt.Println(&fib.fc, &fib.fd)
}

func main() {
	// stressTest()
	var n int
	fmt.Scanf("%d", &n)
	fibonacciFast := newFibonacci(n)
	if 0 <= n && n <= 45 {
		fmt.Printf("%d\n", fibNaive(n))
		fmt.Printf("%d\n", fibonacciFast.nth())
	}

}
