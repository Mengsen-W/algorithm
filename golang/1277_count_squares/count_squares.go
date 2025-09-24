// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countSquares(matrix [][]int) int {
	m, n := len(matrix), len(matrix[0])
	f := make([][]int, m)
	for i := range f {
		f[i] = make([]int, n)
	}
	ans := 0
	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			if i == 0 || j == 0 {
				f[i][j] = matrix[i][j]
			} else if matrix[i][j] == 0 {
				f[i][j] = 0
			} else {
				f[i][j] = min(min(f[i][j-1], f[i-1][j]), f[i-1][j-1]) + 1
			}
			ans += f[i][j]
		}
	}
	return ans
}

func main() {
	tests := []struct {
		matrix [][]int
		ans    int
	}{
		{[][]int{{0, 1, 1, 1}, {1, 1, 1, 1}, {0, 1, 1, 1}}, 15},
		{[][]int{{1, 0, 1}, {1, 1, 0}, {1, 1, 0}}, 7},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countSquares(test.matrix), index)
	}
}
