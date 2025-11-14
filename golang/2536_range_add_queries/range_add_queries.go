// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func rangeAddQueries(n int, queries [][]int) [][]int {
	diff := make([][]int, n+1)
	for i := range diff {
		diff[i] = make([]int, n+1)
	}

	for _, q := range queries {
		row1, col1, row2, col2 := q[0], q[1], q[2], q[3]
		diff[row1][col1] += 1
		diff[row2+1][col1] -= 1
		diff[row1][col2+1] -= 1
		diff[row2+1][col2+1] += 1
	}

	mat := make([][]int, n)
	for i := range mat {
		mat[i] = make([]int, n)
	}

	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			x1 := 0
			if i > 0 {
				x1 = mat[i-1][j]
			}
			x2 := 0
			if j > 0 {
				x2 = mat[i][j-1]
			}
			x3 := 0
			if i > 0 && j > 0 {
				x3 = mat[i-1][j-1]
			}
			mat[i][j] = diff[i][j] + x1 + x2 - x3
		}
	}
	return mat
}

func main() {
	tests := []struct {
		n       int
		queries [][]int
		ans     [][]int
	}{
		{3, [][]int{{1, 1, 2, 2}, {0, 0, 1, 1}}, [][]int{{1, 1, 0}, {1, 2, 1}, {0, 1, 1}}},
		{2, [][]int{{0, 0, 1, 1}}, [][]int{{1, 1}, {1, 1}}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, rangeAddQueries(test.n, test.queries), index)
	}
}
