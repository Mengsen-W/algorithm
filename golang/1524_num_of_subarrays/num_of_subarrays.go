// Package main ...
package main

import "strconv"

func numOfSubarrays(arr []int) int {
	const MODULO = 1000000007
	odd, even := 0, 1
	subarrays := 0
	total := 0

	for _, x := range arr {
		total += x
		if total%2 == 0 {
			subarrays = (subarrays + odd) % MODULO
			even++
		} else {
			subarrays = (subarrays + even) % MODULO
			odd++
		}
	}

	return subarrays % MODULO
}

func main() {
	tests := []struct {
		arr []int
		ans int
	}{
		{[]int{1, 3, 5}, 4},
		{[]int{2, 4, 6}, 0},
		{[]int{1, 2, 3, 4, 5, 6, 7}, 16},
		{[]int{100, 100, 99, 99}, 4},
		{[]int{7}, 1},
	}

	for _, test := range tests {
		if got := numOfSubarrays(test.arr); got != test.ans {
			panic("got " + strconv.Itoa(got) + " want " + strconv.Itoa(test.ans))
		}
	}
}
