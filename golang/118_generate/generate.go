// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func generate(numRows int) [][]int {
	ans := make([][]int, numRows)
	for i := range ans {
		ans[i] = make([]int, i+1)
		ans[i][0] = 1
		ans[i][i] = 1
		for j := 1; j < i; j++ {
			ans[i][j] = ans[i-1][j] + ans[i-1][j-1]
		}
	}
	return ans
}

func main() {
	tests := []struct {
		numRows int
		ans     [][]int
	}{
		{5, [][]int{{1}, {1, 1}, {1, 2, 1}, {1, 3, 3, 1}, {1, 4, 6, 4, 1}}},
		{1, [][]int{{1}}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, generate(test.numRows), index)
	}
}
