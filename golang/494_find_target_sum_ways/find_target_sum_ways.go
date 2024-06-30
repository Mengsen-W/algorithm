// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func findTargetSumWays(nums []int, target int) int {
	sum := 0
	for _, v := range nums {
		sum += v
	}
	diff := sum - target
	if diff < 0 || diff%2 == 1 {
		return 0
	}
	neg := diff / 2
	dp := make([]int, neg+1)
	dp[0] = 1
	for _, num := range nums {
		for j := neg; j >= num; j-- {
			dp[j] += dp[j-num]
		}
	}
	return dp[neg]
}

func main() {
	tests := []struct {
		nums   []int
		target int
		ans    int
	}{
		{[]int{1, 1, 1, 1, 1}, 3, 5},
		{[]int{1}, 1, 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, findTargetSumWays(test.nums, test.target), index)
	}
}
