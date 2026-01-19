// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxSideLength(mat [][]int, threshold int) int {
	m, n := len(mat), len(mat[0])
	P := make([][]int, m+1)
	for i := range P {
		P[i] = make([]int, n+1)
	}

	for i := 1; i <= m; i++ {
		for j := 1; j <= n; j++ {
			P[i][j] = P[i-1][j] + P[i][j-1] - P[i-1][j-1] + mat[i-1][j-1]
		}
	}

	getRect := func(x1, y1, x2, y2 int) int {
		return P[x2][y2] - P[x1-1][y2] - P[x2][y1-1] + P[x1-1][y1-1]
	}

	r := min(m, n)
	ans := 0
	for i := 1; i <= m; i++ {
		for j := 1; j <= n; j++ {
			for c := ans + 1; c <= r; c++ {
				if i+c-1 <= m && j+c-1 <= n && getRect(i, j, i+c-1, j+c-1) <= threshold {
					ans = c
				} else {
					break
				}
			}
		}
	}
	return ans
}

func main() {
	tests := []struct {
		mat       [][]int
		threshold int
		ans       int
	}{
		{[][]int{{1, 1, 3, 2, 4, 3, 2}, {1, 1, 3, 2, 4, 3, 2}, {1, 1, 3, 2, 4, 3, 2}}, 4, 2},
		{[][]int{{2, 2, 2, 2, 2}, {2, 2, 2, 2, 2}, {2, 2, 2, 2, 2}, {2, 2, 2, 2, 2}, {2, 2, 2, 2, 2}}, 1, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxSideLength(test.mat, test.threshold), index)
	}
}
