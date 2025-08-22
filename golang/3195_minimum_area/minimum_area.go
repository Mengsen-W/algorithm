// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumArea(grid [][]int) int {
	n, m := len(grid), len(grid[0])
	minI, maxI := n, 0
	minJ, maxJ := m, 0
	for i := 0; i < n; i++ {
		for j := 0; j < m; j++ {
			if grid[i][j] == 1 {
				minI = min(minI, i)
				maxI = max(maxI, i)
				minJ = min(minJ, j)
				maxJ = max(maxJ, j)
			}
		}
	}
	return (maxI - minI + 1) * (maxJ - minJ + 1)
}

func main() {
	tests := []struct {
		grid [][]int
		ans  int
	}{
		{[][]int{{0, 1, 0}, {1, 0, 1}}, 6},
		{[][]int{{0, 0}, {1, 0}}, 1},
	}
	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimumArea(test.grid), index)
	}
}
