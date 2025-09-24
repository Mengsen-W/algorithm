// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func sortMatrix(grid [][]int) [][]int {
	n := len(grid)

	for i := 0; i < n; i++ {
		tmp := []int{}
		for j := 0; i+j < n; j++ {
			tmp = append(tmp, grid[i+j][j])
		}
		sort.Sort(sort.Reverse(sort.IntSlice(tmp)))
		for j := 0; i+j < n; j++ {
			grid[i+j][j] = tmp[j]
		}
	}

	for j := 1; j < n; j++ {
		tmp := []int{}
		for i := 0; j+i < n; i++ {
			tmp = append(tmp, grid[i][j+i])
		}
		sort.Ints(tmp)
		for i := 0; j+i < n; i++ {
			grid[i][j+i] = tmp[i]
		}
	}

	return grid
}

func main() {
	tests := []struct {
		grid   [][]int
		output [][]int
	}{
		{[][]int{{1, 7, 3}, {9, 8, 2}, {4, 5, 6}}, [][]int{{8, 2, 3}, {9, 6, 7}, {4, 5, 1}}},
		{[][]int{{0, 1}, {1, 2}}, [][]int{{2, 1}, {1, 0}}},
		{[][]int{{1}}, [][]int{{1}}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.output, sortMatrix(test.grid), index)
	}
}
