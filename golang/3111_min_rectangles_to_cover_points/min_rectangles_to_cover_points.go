// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minRectanglesToCoverPoints(points [][]int, w int) int {
	sort.Slice(points, func(i, j int) bool {
		return points[i][0] < points[j][0]
	})
	res := 0
	bound := -1
	for _, p := range points {
		if p[0] > bound {
			bound = p[0] + w
			res++
		}
	}
	return res
}

func main() {
	tests := []struct {
		points [][]int
		w      int
		ans    int
	}{
		{[][]int{{2, 1}, {1, 0}, {1, 4}, {1, 8}, {3, 5}, {4, 6}}, 1, 2},
		{[][]int{{0, 0}, {1, 1}, {2, 2}, {3, 3}, {4, 4}, {5, 5}, {6, 6}}, 2, 3},
		{[][]int{{2, 3}, {1, 2}}, 0, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minRectanglesToCoverPoints(test.points, test.w), index)
	}
}
