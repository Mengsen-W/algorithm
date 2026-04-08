// Package main ...
package main

import "fmt"

func xorAfterQueries(nums []int, queries [][]int) int {
	const mod = 1_000_000_007

	for _, q := range queries {
		l, r, k, v := q[0], q[1], q[2], q[3]
		for i := l; i <= r; i += k {
			nums[i] = int((int64(nums[i]) * int64(v)) % mod)
		}
	}

	res := 0
	for _, x := range nums {
		res ^= x
	}
	return res
}

func main() {
	tests := []struct {
		nums    []int
		queries [][]int
		ans     int
	}{
		{[]int{1, 1, 1}, [][]int{{0, 2, 1, 4}}, 4},
		{[]int{2, 3, 1, 5, 4}, [][]int{{1, 4, 2, 3}, {0, 2, 1, 2}}, 31},
	}

	for _, test := range tests {
		if xorAfterQueries(test.nums, test.queries) != test.ans {
			fmt.Print(test)
		}
	}
}
