// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func differenceOfDistinctValues(grid [][]int) [][]int {
	abs := func(x int) int {
		if x < 0 {
			return -x
		}
		return x
	}

	m := len(grid)
	n := len(grid[0])
	res := make([][]int, m)
	for i := range res {
		res[i] = make([]int, n)
	}

	for i := 0; i < m; i++ {
		x, y := i, 0
		s := make(map[int]bool)
		for x < m && y < n {
			res[x][y] += len(s)
			s[grid[x][y]] = true
			x++
			y++
		}
	}

	for j := 1; j < n; j++ {
		x, y := 0, j
		s := make(map[int]bool)
		for x < m && y < n {
			res[x][y] += len(s)
			s[grid[x][y]] = true
			x++
			y++
		}
	}

	for i := 0; i < m; i++ {
		x, y := i, n-1
		s := make(map[int]bool)
		for x >= 0 && y >= 0 {
			res[x][y] -= len(s)
			res[x][y] = abs(res[x][y])
			s[grid[x][y]] = true
			x--
			y--
		}
	}

	for j := n - 2; j >= 0; j-- {
		x, y := m-1, j
		s := make(map[int]bool)
		for x >= 0 && y >= 0 {
			res[x][y] -= len(s)
			res[x][y] = abs(res[x][y])
			s[grid[x][y]] = true
			x--
			y--
		}
	}

	return res
}

func main() {
	tests := []struct {
		grid [][]int
		ans  [][]int
	}{
		{[][]int{{1, 2, 3}, {3, 1, 5}, {3, 2, 1}}, [][]int{{1, 1, 0}, {1, 0, 1}, {0, 1, 1}}},
		{[][]int{{1}}, [][]int{{0}}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, differenceOfDistinctValues(test.grid), index)
	}
}
