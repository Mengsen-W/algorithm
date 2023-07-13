/*
 * @Date: 2023-07-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-13
 * @FilePath: /algorithm/golang/931_min_falling_path_sum/min_falling_path_sum.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minFallingPathSum(matrix [][]int) int {
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}

	n := len(matrix)
	dp := make([][]int, n)
	for i := range dp {
		dp[i] = make([]int, n)
	}
	copy(dp[0], matrix[0])
	for i := 1; i < n; i++ {
		for j := 0; j < n; j++ {
			mn := dp[i-1][j]
			if j > 0 {
				mn = min(mn, dp[i-1][j-1])
			}
			if j < n-1 {
				mn = min(mn, dp[i-1][j+1])
			}
			dp[i][j] = mn + matrix[i][j]
		}
	}
	minVal := dp[n-1][0]
	for _, val := range dp[n-1] {
		if val < minVal {
			minVal = val
		}
	}
	return minVal
}

func main() {
	tests := []struct {
		matrix [][]int
		ans    int
	}{
		{[][]int{{2, 1, 3}, {6, 5, 4}, {7, 8, 9}}, 13},
		{[][]int{{-19, 57}, {-40, -5}}, -59},
	}

	for _, item := range tests {
		assert.Equal(&testing.T{}, minFallingPathSum(item.matrix), item.ans)
	}
}
