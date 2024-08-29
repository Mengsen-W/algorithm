// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func satisfiesConditions(grid [][]int) bool {
	for i := range grid {
		for j := range grid[0] {
			if i+1 < len(grid) && grid[i][j] != grid[i+1][j] {
				return false
			}
			if j+1 < len(grid[0]) && grid[i][j] == grid[i][j+1] {
				return false
			}
		}
	}
	return true
}

func main() {
	tests := []struct {
		grid [][]int
		ans  bool
	}{
		{[][]int{{1, 0, 2}, {1, 0, 2}}, true},
		{[][]int{{1, 1, 1}, {0, 0, 0}}, false},
		{[][]int{{1}, {2}, {3}}, false},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, satisfiesConditions(test.grid), index)
	}
}
