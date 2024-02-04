/*
 * @Date: 2024-02-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-03
 * @FilePath: /algorithm/golang/1690_window.native/window.native.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func stoneGameVII(stones []int) int {
	n := len(stones)
	sum := make([]int, n+1)
	dp := make([][]int, n)

	for i := range dp {
		dp[i] = make([]int, n)
	}
	for i := 0; i < n; i++ {
		sum[i+1] = sum[i] + stones[i]
	}
	for i := n - 2; i >= 0; i-- {
		for j := i + 1; j < n; j++ {
			dp[i][j] = max(sum[j+1]-sum[i+1]-dp[i+1][j], sum[j]-sum[i]-dp[i][j-1])
		}
	}
	return dp[0][n-1]
}

func main() {
	tests := []struct {
		stones []int
		ans    int
	}{
		{[]int{5, 3, 1, 4, 2}, 6},
		{[]int{7, 90, 5, 1, 100, 10, 10, 2}, 122},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, stoneGameVII(test.stones), index)
	}
}
