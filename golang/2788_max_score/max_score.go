// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxScore(nums []int, x int) int64 {
	res := int64(nums[0])
	dp := [2]int64{math.MinInt32, math.MinInt32}
	dp[nums[0]%2] = int64(nums[0])
	for i := 1; i < len(nums); i++ {
		parity := nums[i] % 2
		cur := max(dp[parity]+int64(nums[i]), dp[1-parity]-int64(x)+int64(nums[i]))
		res = max(res, cur)
		dp[parity] = max(dp[parity], cur)
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		x    int
		ans  int64
	}{
		{[]int{2, 3, 6, 1, 9, 2}, 5, 13},
		{[]int{2, 4, 6, 8}, 3, 20},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxScore(test.nums, test.x), index)
	}
}
