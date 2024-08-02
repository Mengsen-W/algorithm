// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func numberOfRightTriangles(grid [][]int) int64 {
	n := len(grid)
	m := len(grid[0])
	col := make([]int, m)
	for i := 0; i < n; i++ {
		for j := 0; j < m; j++ {
			col[j] += grid[i][j]
		}
	}

	var res int64
	for i := 0; i < n; i++ {
		row := 0
		for j := 0; j < m; j++ {
			row += grid[i][j]
		}
		for j := 0; j < m; j++ {
			if grid[i][j] == 1 {
				res += int64(row-1) * int64(col[j]-1)
			}
		}
	}
	return res
}

func main() {
	tests := []struct {
		grid [][]int
		ans  int64
	}{
		{[][]int{{0, 1, 0}, {0, 1, 1}, {0, 1, 0}}, 2},
		{[][]int{{1, 0, 0, 0}, {0, 1, 0, 1}, {1, 0, 0, 0}}, 0},
		{[][]int{{1, 0, 1}, {1, 0, 0}, {1, 0, 0}}, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, numberOfRightTriangles(test.grid), index)
	}
}
