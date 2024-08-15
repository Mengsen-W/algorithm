// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxScore(grid [][]int) int {
	m := len(grid)
	n := len(grid[0])
	precol := make([]int, n)
	for i := range precol {
		precol[i] = math.MinInt32
	}
	ans := math.MinInt32

	for i := 0; i < m; i++ {
		prerow := math.MinInt32
		for j := 0; j < n; j++ {
			f := math.MinInt32
			if i > 0 {
				f = max(f, grid[i][j]+precol[j])
			}
			if j > 0 {
				f = max(f, grid[i][j]+prerow)
			}
			ans = max(ans, f)
			precol[j] = max(precol[j], max(f, 0)-grid[i][j])
			prerow = max(prerow, max(f, 0)-grid[i][j])
		}
	}

	return ans
}

func main() {
	tests := []struct {
		grid [][]int
		ans  int
	}{
		{[][]int{{9, 5, 7, 3}, {8, 9, 6, 1}, {6, 7, 14, 3}, {2, 5, 3, 1}}, 9},
		{[][]int{{4, 3, 2}, {3, 2, 1}}, -1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxScore(test.grid), index)
	}
}
