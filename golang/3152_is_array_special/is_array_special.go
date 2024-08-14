// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func isArraySpecial(nums []int, queries [][]int) []bool {
	n := len(nums)
	dp := make([]int, n)
	for i := 0; i < n; i++ {
		dp[i] = 1
	}
	for i := 1; i < n; i++ {
		if (nums[i]^nums[i-1])&1 == 1 {
			dp[i] = dp[i-1] + 1
		}
	}

	res := make([]bool, len(queries))
	for i, q := range queries {
		x, y := q[0], q[1]
		res[i] = dp[y] >= y-x+1
	}
	return res
}

func main() {
	tests := []struct {
		nums    []int
		queries [][]int
		ans     []bool
	}{
		{[]int{3, 4, 1, 2, 6}, [][]int{{0, 4}}, []bool{false}},
		{[]int{4, 3, 1, 6}, [][]int{{0, 2}, {2, 3}}, []bool{false, true}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, isArraySpecial(test.nums, test.queries), index)
	}
}
