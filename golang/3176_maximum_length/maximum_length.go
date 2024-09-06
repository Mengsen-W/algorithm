// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximumLength(nums []int, k int) int {
	lenNums := len(nums)
	dp := make([][]int, lenNums)
	for i := range dp {
		dp[i] = make([]int, 51)
		for j := range dp[i] {
			dp[i][j] = -1
		}
	}

	ans := 0

	for i := 0; i < lenNums; i++ {
		dp[i][0] = 1
		for l := 0; l <= k; l++ {
			for j := 0; j < i; j++ {
				add := 0
				if nums[i] != nums[j] {
					add = 1
				}
				if l-add >= 0 && dp[j][l-add] != -1 {
					dp[i][l] = max(dp[i][l], dp[j][l-add]+1)
				}
			}
			ans = max(ans, dp[i][l])
		}
	}

	return ans
}

func main() {
	tests := []struct {
		nums []int
		k    int
		ans  int
	}{
		{[]int{1, 2, 1, 1, 3}, 2, 4},
		{[]int{1, 2, 3, 4, 5, 1}, 0, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maximumLength(test.nums, test.k), index)
	}
}
