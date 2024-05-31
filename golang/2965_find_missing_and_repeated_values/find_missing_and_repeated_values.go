// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func findMissingAndRepeatedValues(grid [][]int) []int {
	n := len(grid)
	count := make([]int, n*n+1)
	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			count[grid[i][j]]++
		}
	}

	res := make([]int, 2)
	for i := 1; i <= n*n; i++ {
		if count[i] == 2 {
			res[0] = i
		}
		if count[i] == 0 {
			res[1] = i
		}
	}
	return res
}

func main() {
	tests := []struct {
		grid [][]int
		ans  []int
	}{
		{[][]int{{1, 3}, {2, 2}}, []int{2, 4}},
		{[][]int{{9, 1, 7}, {8, 9, 2}, {3, 4, 6}}, []int{9, 5}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, findMissingAndRepeatedValues(test.grid), index)
	}
}
