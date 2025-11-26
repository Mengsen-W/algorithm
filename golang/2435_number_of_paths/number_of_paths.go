// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func numberOfPaths(grid [][]int, k int) int {
	const MOD = 1000000007
	m, n := len(grid), len(grid[0])

	dp := make([][][]int64, m+1)
	for i := range dp {
		dp[i] = make([][]int64, n+1)
		for j := range dp[i] {
			dp[i][j] = make([]int64, k)
		}
	}

	for i := 1; i <= m; i++ {
		for j := 1; j <= n; j++ {
			if i == 1 && j == 1 {
				dp[i][j][grid[0][0]%k] = 1
				continue
			}

			value := grid[i-1][j-1] % k
			for r := 0; r < k; r++ {
				prevMod := (r - value + k) % k
				dp[i][j][r] = (dp[i-1][j][prevMod] + dp[i][j-1][prevMod]) % MOD
			}
		}
	}

	return int(dp[m][n][0])
}

func main() {
	tests := []struct {
		grid [][]int
		k    int
		ans  int
	}{
		{[][]int{{5, 2, 4}, {3, 0, 5}, {0, 7, 2}}, 3, 2},
		{[][]int{{0, 0}}, 5, 1},
		{[][]int{{7, 3, 4, 9}, {2, 3, 6, 2}, {2, 3, 7, 0}}, 1, 10},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, numberOfPaths(test.grid, test.k), index)
	}
}
