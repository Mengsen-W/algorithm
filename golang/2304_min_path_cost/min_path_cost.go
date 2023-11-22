/*
 * @Date: 2023-11-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-22
 * @FilePath: /algorithm/golang/2304_min_path_cost/min_path_cost.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minPathCost(grid [][]int, moveCost [][]int) int {
	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}

	const inf = 0x3f3f3f3f
	m, n, cur := len(grid), len(grid[0]), 0
	dp := [2][]int{
		make([]int, n), make([]int, n),
	}
	copy(dp[0], grid[0])
	for i := 1; i < m; i++ {
		next := 1 - cur
		for j := 0; j < n; j++ {
			dp[next][j] = inf
			for k := 0; k < n; k++ {
				dp[next][j] = min(dp[next][j], dp[cur][k]+moveCost[grid[i-1][k]][j]+grid[i][j])
			}
		}
		cur = next
	}
	res := inf
	for j := 0; j < n; j++ {
		res = min(res, dp[cur][j])
	}
	return res
}

func main() {
	tests := []struct {
		grid, moveCost [][]int
		ans            int
	}{
		{[][]int{{5, 3}, {4, 0}, {2, 1}}, [][]int{{9, 8}, {1, 5}, {10, 12}, {18, 6}, {2, 4}, {14, 3}}, 17},
		{[][]int{{5, 1, 2}, {4, 0, 3}}, [][]int{{12, 10, 15}, {20, 23, 8}, {21, 7, 1}, {8, 1, 13}, {9, 10, 25}, {5, 3, 2}}, 6},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minPathCost(test.grid, test.moveCost), index)
	}
}
