// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minTimeToVisitAllPoints(points [][]int) int {
	abs := func(x int) int {
		if x < 0 {
			return -x
		}
		return x
	}

	x0, y0 := points[0][0], points[0][1]
	ans := 0
	for i := 1; i < len(points); i++ {
		x1, y1 := points[i][0], points[i][1]
		dx := abs(x0 - x1)
		dy := abs(y0 - y1)
		if dx > dy {
			ans += dx
		} else {
			ans += dy
		}
		x0, y0 = x1, y1
	}
	return ans
}

func main() {
	tests := []struct {
		points [][]int
		ans    int
	}{
		{[][]int{{1, 1}, {3, 4}, {-1, 0}}, 7},
		{[][]int{{3, 2}, {-2, 2}}, 5},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minTimeToVisitAllPoints(test.points), index)
	}
}
