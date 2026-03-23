// Package main ...
package main

import "fmt"

func maxProductPath(grid [][]int) int {
	const MOD = 1000000007
	m, n := len(grid), len(grid[0])

	maxgt := make([][]int64, m)
	minlt := make([][]int64, m)
	for i := range maxgt {
		maxgt[i] = make([]int64, n)
		minlt[i] = make([]int64, n)
	}

	maxgt[0][0] = int64(grid[0][0])
	minlt[0][0] = int64(grid[0][0])
	for i := 1; i < n; i++ {
		maxgt[0][i] = maxgt[0][i-1] * int64(grid[0][i])
		minlt[0][i] = maxgt[0][i]
	}
	for i := 1; i < m; i++ {
		maxgt[i][0] = maxgt[i-1][0] * int64(grid[i][0])
		minlt[i][0] = maxgt[i][0]
	}
	for i := 1; i < m; i++ {
		for j := 1; j < n; j++ {
			if grid[i][j] >= 0 {
				maxPrev := max(maxgt[i][j-1], maxgt[i-1][j])
				minPrev := min(minlt[i][j-1], minlt[i-1][j])
				maxgt[i][j] = maxPrev * int64(grid[i][j])
				minlt[i][j] = minPrev * int64(grid[i][j])
			} else {
				maxPrev := max(maxgt[i][j-1], maxgt[i-1][j])
				minPrev := min(minlt[i][j-1], minlt[i-1][j])
				maxgt[i][j] = minPrev * int64(grid[i][j])
				minlt[i][j] = maxPrev * int64(grid[i][j])
			}
		}
	}

	if maxgt[m-1][n-1] < 0 {
		return -1
	}
	return int(maxgt[m-1][n-1] % MOD)
}

func main() {
	tests := []struct {
		grid [][]int
		ans  int
	}{
		{[][]int{{-1, -2, -3}, {-2, -3, -3}, {-3, -3, -2}}, -1},
		{[][]int{{1, -2, 1}, {1, -2, 1}, {3, -4, 1}}, 8},
		{[][]int{{1, 3}, {0, -4}}, 0},
	}

	for _, test := range tests {
		if maxProductPath(test.grid) != test.ans {
			panic(fmt.Sprintf("test failed: expected %d, got %d", test.ans, maxProductPath(test.grid)))
		}
	}
}
