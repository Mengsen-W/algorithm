// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countNegatives(grid [][]int) int {
	num := 0
	m := len(grid[0])
	pos := len(grid[0]) - 1

	for _, row := range grid {
		i := pos
		for ; i >= 0; i-- {
			if row[i] >= 0 {
				if i+1 < m {
					pos = i + 1
					num += m - pos
				}
				break
			}
		}
		if i == -1 {
			num += m
			pos = -1
		}
	}

	return num
}

func main() {
	tests := []struct {
		grid [][]int
		ans  int
	}{
		{[][]int{{4, 3, 2, -1}, {3, 2, 1, -1}, {1, 1, -1, -2}, {-1, -1, -2, -3}}, 8},
		{[][]int{{3, 2}, {1, 0}}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countNegatives(test.grid), index)
	}
}
