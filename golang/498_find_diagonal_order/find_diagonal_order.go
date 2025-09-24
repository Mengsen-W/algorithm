// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func findDiagonalOrder(mat [][]int) []int {
	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}

	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}
	m, n := len(mat), len(mat[0])
	ans := make([]int, 0, m*n)
	for i := 0; i < m+n-1; i++ {
		if i%2 == 1 {
			x := max(i-n+1, 0)
			y := min(i, n-1)
			for x < m && y >= 0 {
				ans = append(ans, mat[x][y])
				x++
				y--
			}
		} else {
			x := min(i, m-1)
			y := max(i-m+1, 0)
			for x >= 0 && y < n {
				ans = append(ans, mat[x][y])
				x--
				y++
			}
		}
	}
	return ans
}

func main() {
	tests := []struct {
		mat    [][]int
		expect []int
	}{
		{[][]int{{1, 2, 3}, {4, 5, 6}, {7, 8, 9}}, []int{1, 2, 4, 7, 5, 3, 6, 8, 9}},
		{[][]int{{1, 2}, {3, 4}}, []int{1, 2, 3, 4}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.expect, findDiagonalOrder(test.mat), index)
	}
}
