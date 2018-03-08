package main

func oneTo200k() []int {
	dat := make([]int, 200000)
	n := 1
	for i := 0; i < 200000; i++ {
		dat[i] = n
		n += 1
	}
	return dat
}
