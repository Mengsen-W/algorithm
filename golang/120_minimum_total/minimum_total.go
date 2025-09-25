// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumTotal(triangle [][]int) int {
	min := func(x, y int) int {
		if x < y {
			return x
		}
		return y
	}
	n := len(triangle)
	f := make([]int, n)
	f[0] = triangle[0][0]
	for i := 1; i < n; i++ {
		f[i] = f[i-1] + triangle[i][i]
		for j := i - 1; j > 0; j-- {
			f[j] = min(f[j-1], f[j]) + triangle[i][j]
		}
		f[0] += triangle[i][0]
	}
	ans := math.MaxInt32
	for i := 0; i < n; i++ {
		ans = min(ans, f[i])
	}
	return ans
}

func main() {
	tests := []struct {
		triangle [][]int
		ans      int
	}{
		{[][]int{{2}, {3, 4}, {6, 5, 7}, {4, 1, 8, 3}}, 11},
		{[][]int{{-10}}, -10},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimumTotal(test.triangle), index)
	}
}
