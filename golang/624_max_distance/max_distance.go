// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxDistance(arrays [][]int) int {
	abs := func(x int) int {
		if x < 0 {
			return -x
		}
		return x
	}
	res := 0
	n := len(arrays[0])
	minVal := arrays[0][0]
	maxVal := arrays[0][n-1]
	for i := 1; i < len(arrays); i++ {
		n = len(arrays[i])
		res = max(res, max(abs(arrays[i][n-1]-minVal),
			abs(maxVal-arrays[i][0])))
		minVal = min(minVal, arrays[i][0])
		maxVal = max(maxVal, arrays[i][n-1])
	}
	return res
}

func main() {
	tests := []struct {
		arrays [][]int
		ans    int
	}{
		{[][]int{{1, 2, 3}, {4, 5}, {1, 2, 3}}, 4},
		{[][]int{{1}, {1}}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxDistance(test.arrays), index)
	}
}
