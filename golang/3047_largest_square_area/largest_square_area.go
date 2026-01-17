// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func largestSquareArea(bottomLeft [][]int, topRight [][]int) int64 {
	n := len(bottomLeft)
	maxSide := int64(0)

	for i := 0; i < n; i++ {
		for j := i + 1; j < n; j++ {
			w := min(topRight[i][0], topRight[j][0]) - max(bottomLeft[i][0], bottomLeft[j][0])
			h := min(topRight[i][1], topRight[j][1]) - max(bottomLeft[i][1], bottomLeft[j][1])

			if w > 0 && h > 0 {
				side := min(w, h)
				if int64(side) > maxSide {
					maxSide = int64(side)
				}
			}
		}
	}

	return maxSide * maxSide
}

func main() {
	tests := []struct {
		bottomLeft [][]int
		topRight   [][]int
		expected   int64
	}{
		{[][]int{{1, 1}, {2, 2}, {3, 1}}, [][]int{{3, 3}, {4, 4}, {6, 6}}, 1},
		{[][]int{{1, 1}, {2, 2}, {1, 2}}, [][]int{{3, 3}, {4, 4}, {3, 4}}, 1},
		{[][]int{{1, 1}, {3, 3}, {3, 1}}, [][]int{{2, 2}, {4, 4}, {4, 2}}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.expected, largestSquareArea(test.bottomLeft, test.topRight), index)
	}
}
