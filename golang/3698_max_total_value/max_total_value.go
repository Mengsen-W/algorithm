// Package main ...
package main

import "math"

func maxTotalValue(nums []int, k int) int64 {
	m1, m2 := math.MaxInt, math.MinInt
	for _, x := range nums {
		m1 = min(m1, x)
		m2 = max(m2, x)
	}
	return int64(m2-m1) * int64(k)
}

func main() {
	tests := []struct {
		nums []int
		k    int
		ans  int64
	}{
		{[]int{1, 3, 2}, 2, 4},
		{[]int{4, 2, 5, 1}, 3, 12},
	}

	for _, test := range tests {
		if maxTotalValue(test.nums, test.k) != test.ans {
			println("test failed")
		}
	}
}
