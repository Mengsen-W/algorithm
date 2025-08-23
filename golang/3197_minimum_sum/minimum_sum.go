// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumSum2(grid [][]int, u, d, l, r int) int {
	minI, maxI := len(grid), 0
	minJ, maxJ := len(grid[0]), 0
	for i := u; i <= d; i++ {
		for j := l; j <= r; j++ {
			if grid[i][j] == 1 {
				minI = min(minI, i)
				minJ = min(minJ, j)
				maxI = max(maxI, i)
				maxJ = max(maxJ, j)
			}
		}
	}
	if minI <= maxI {
		return (maxI - minI + 1) * (maxJ - minJ + 1)
	}
	return math.MaxInt32 / 3
}

func rotate(vec [][]int) [][]int {
	n, m := len(vec), len(vec[0])
	ret := make([][]int, m)
	for i := range ret {
		ret[i] = make([]int, n)
	}
	for i := 0; i < n; i++ {
		for j := 0; j < m; j++ {
			ret[m-j-1][i] = vec[i][j]
		}
	}
	return ret
}

func solve(grid [][]int) int {
	n, m := len(grid), len(grid[0])
	res := n * m
	for i := 0; i+1 < n; i++ {
		for j := 0; j+1 < m; j++ {
			res = min(res, minimumSum2(grid, 0, i, 0, m-1)+
				minimumSum2(grid, i+1, n-1, 0, j)+
				minimumSum2(grid, i+1, n-1, j+1, m-1))
			res = min(res, minimumSum2(grid, 0, i, 0, j)+
				minimumSum2(grid, 0, i, j+1, m-1)+
				minimumSum2(grid, i+1, n-1, 0, m-1))
		}
	}
	for i := 0; i+2 < n; i++ {
		for j := i + 1; j+1 < n; j++ {
			res = min(res, minimumSum2(grid, 0, i, 0, m-1)+
				minimumSum2(grid, i+1, j, 0, m-1)+
				minimumSum2(grid, j+1, n-1, 0, m-1))
		}
	}
	return res
}

func minimumSum(grid [][]int) int {
	rgrid := rotate(grid)
	return min(solve(grid), solve(rgrid))
}

func main() {
	tests := []struct {
		grid [][]int
		ans  int
	}{
		{[][]int{{1, 0, 1}, {1, 1, 1}}, 5},
		{[][]int{{1, 0, 1, 0}, {0, 1, 0, 1}}, 5},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimumSum(test.grid), index)
	}
}
