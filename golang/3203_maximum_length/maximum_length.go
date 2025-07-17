// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximumLength(nums []int, k int) int {
	dp := make([][]int, k)
	for i := range dp {
		dp[i] = make([]int, k)
	}
	res := 0
	for _, num := range nums {
		num %= k
		for prev := 0; prev < k; prev++ {
			dp[prev][num] = dp[num][prev] + 1
			res = max(res, dp[prev][num])
		}
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		k    int
		ans  int
	}{
		{[]int{1, 2, 3, 4, 5}, 2, 5},
		{[]int{1, 4, 2, 3, 1, 4}, 3, 4},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maximumLength(test.nums, test.k), "case %d", index)
	}
}
