// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxPointsInsideSquare(points [][]int, s string) int {
	min1 := make([]int, 26)
	for i := range min1 {
		min1[i] = 1000000001
	}
	min2 := 1000000001
	for i, ch := range s {
		x, y := points[i][0], points[i][1]
		j := int(ch - 'a')
		d := max(abs(x), abs(y))
		if d < min1[j] {
			min2 = min(min2, min1[j])
			min1[j] = d
		} else if d < min2 {
			min2 = d
		}
	}
	res := 0
	for _, d := range min1 {
		if d < min2 {
			res++
		}
	}
	return res
}

func abs(a int) int {
	if a > 0 {
		return a
	}
	return -a
}

func main() {
	tests := []struct {
		points [][]int
		s      string
		ans    int
	}{
		{[][]int{{2, 2}, {-1, -2}, {-4, 4}, {-3, 1}, {3, -3}}, "abdca", 2},
		{[][]int{{1, 1}, {-1, -1}, {2, -2}}, "ccd", 0},
		{[][]int{{1, 1}, {-2, -2}, {-2, 2}}, "abb", 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxPointsInsideSquare(test.points, test.s), index)
	}
}
