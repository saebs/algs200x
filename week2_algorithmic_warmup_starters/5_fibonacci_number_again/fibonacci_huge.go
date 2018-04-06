package main

import (
	"fmt"
	// "math"
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
	n    int
	m    int
	mBig big.Int
	/* fibonacci 2x2 and Q matrices
	   [a b]
	   [c d]
	*/

	fa, fb, fc, fd big.Int
	qa, qb, qc, qd big.Int
}

func newFibonacci(n, m int) fibonacci {
	// this is our constructor
	f := fibonacci{}
	f.n = n
	f.m = m
	f.mBig.SetInt64(int64(m))
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
func (fib *fibonacci) fnModm() *big.Int {
	var fibmod big.Int
	if fib.n <= 1 {
		return big.NewInt(int64(fib.n))
	}
	fib.remainder() // get remainder of Large Fn divided by pisano period
	fib.power()
	return fibmod.Mod(&fib.fa, &fib.mBig)
}

func (fib *fibonacci) remainder() {
	// multiply matrix by itself n-1 times
	// and determines Pissano period length
	// out puts new smaller equivalent if Large N
	// multiply matrix by itself n-1 times
	// from the matrix formula
	// [ Fn+1  Fn   ]
	// [ Fn    Fn-1 ]

	// finding fibonacci period start where Fn%m = 0 and Fn+1%m = 1
	// f
	pisanoPeriod := 1

	for i := 2; i <= fib.n-1; i++ {
		fib.multiply()
		// fmt.Println(i)

		if fib.fa.Int64()%int64(fib.m) == int64(1) && fib.fb.Int64()%int64(fib.m) == int64(0) {
			pisanoPeriod = i
			fmt.Println("fin", pisanoPeriod)
			break
		}
	}
	// remainder of Fn divided by pisano
	fib.n = fib.n % pisanoPeriod
}

// Helper function calculates fibonaci matrix "f" to power n

func (fib *fibonacci) power() {
	// multiply matrix by itself n-1 times
	//find pissano period, by obsevation last count is the length of period, NB: period starts with 0 1, in modulo
	for i := 2; i <= fib.n-1; i++ {
		fib.multiply()
	}
}

// 2x2 matrix multiplier
func (fib *fibonacci) multiply() {
	var a, b, c, d, x, y big.Int
	// filling fibonacci matrix
	a.Add(x.Mul(&fib.fa, &fib.qa), y.Mul(&fib.fb, &fib.qc))
	b.Add(x.Mul(&fib.fa, &fib.qb), y.Mul(&fib.fb, &fib.qd))
	c.Add(x.Mul(&fib.fc, &fib.qa), y.Mul(&fib.fd, &fib.qc))
	d.Add(x.Mul(&fib.fc, &fib.qb), y.Mul(&fib.fd, &fib.qd))
	fib.fa = a
	fib.fb = b
	fib.fc = c
	fib.fd = d
	// debugging resultant matrix
	// fmt.Println(&fib.fa, &fib.fb)
	// fmt.Println(&fib.fc, &fib.fd)
}

func main() {
	var n, m int
	fmt.Scanf("%d %d", &n, &m)
	fibonacciFast := newFibonacci(n, m)
	// Contraints 1 <= n <= 10^18 , 2 <= m <= 10^5
	if 1 <= n && n <= 1000000000000000000 && 2 <= m && m <= 1000000 {
		// fmt.Printf("%d\n", fibNaive(n))
		fmt.Printf("%d\n", fibonacciFast.fnModm())
	}

}
