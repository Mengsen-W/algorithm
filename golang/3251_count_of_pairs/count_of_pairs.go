// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countOfPairs(nums []int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	n := len(nums)
	m := 0
	for _, num := range nums {
		if num > m {
			m = num
		}
	}
	mod := int(1e9 + 7)
	dp := make([][]int, n)
	for i := range dp {
		dp[i] = make([]int, m+1)
	}
	for a := 0; a <= nums[0]; a++ {
		dp[0][a] = 1
	}
	for i := 1; i < n; i++ {
		d := max(0, nums[i]-nums[i-1])
		for j := d; j <= nums[i]; j++ {
			if j == 0 {
				dp[i][j] = dp[i-1][j-d]
			} else {
				dp[i][j] = (dp[i][j-1] + dp[i-1][j-d]) % mod
			}
		}
	}
	res := 0
	for _, num := range dp[n-1] {
		res = (res + num) % mod
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{2, 3, 2}, 4},
		{[]int{5, 5, 5, 5}, 126},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countOfPairs(test.nums), index)
	}
}
