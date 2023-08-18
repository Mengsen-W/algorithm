/*
 * @Date: 2023-08-18
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-18
 * @FilePath: /algorithm/golang/1388_max_size_slices/max_size_slices.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxSizeSlices(slices []int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	calculate := func(slices []int) int {
		N, n := len(slices), (len(slices)+1)/3
		dp := make([][]int, N)
		for i := 0; i < N; i++ {
			dp[i] = make([]int, n+1)
			for j := 0; j <= n; j++ {
				dp[i][j] = -0x3f3f3f3f
			}
		}
		dp[0][0], dp[0][1], dp[1][0], dp[1][1] = 0, slices[0], 0, max(slices[0], slices[1])
		for i := 2; i < N; i++ {
			dp[i][0] = 0
			for j := 1; j <= n; j++ {
				dp[i][j] = max(dp[i-1][j], dp[i-2][j-1]+slices[i])
			}
		}
		return dp[N-1][n]
	}
	return max(calculate(slices[1:]), calculate(slices[:len(slices)-1]))
}

func main() {
	tests := []struct {
		slices []int
		ans    int
	}{
		{[]int{1, 2, 3, 4, 5, 6}, 10},
		{[]int{8, 9, 8, 6, 1, 1}, 16},
	}

	for _, item := range tests {
		assert.Equal(&testing.T{}, item.ans, maxSizeSlices(item.slices))
	}
}
