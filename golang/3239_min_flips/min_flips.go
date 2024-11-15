// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minFlips(grid [][]int) int {
	rowCnt, colCnt := 0, 0
	m, n := len(grid), len(grid[0])
	for i := 0; i < m; i++ {
		for j1 := 0; j1 < n/2; j1++ {
			j2 := n - 1 - j1
			if grid[i][j1] != grid[i][j2] {
				rowCnt++
			}
		}
	}
	for j := 0; j < n; j++ {
		for i1 := 0; i1 < m/2; i1++ {
			i2 := m - 1 - i1
			if grid[i1][j] != grid[i2][j] {
				colCnt++
			}
		}
	}
	return min(colCnt, rowCnt)
}

func main() {
	tests := []struct {
		grid [][]int
		ans  int
	}{
		{[][]int{{1, 0, 0}, {0, 0, 0}, {0, 0, 1}}, 2},
		{[][]int{{0, 1}, {0, 1}, {0, 0}}, 1},
		{[][]int{{1}, {0}}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minFlips(test.grid), index)
	}
}
