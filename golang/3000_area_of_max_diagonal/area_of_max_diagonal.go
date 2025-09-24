// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func areaOfMaxDiagonal(dimensions [][]int) int {
	maxDiaSq := 0
	maxArea := 0
	for _, dim := range dimensions {
		l := dim[0]
		w := dim[1]
		diaSq := l*l + w*w
		area := l * w
		if diaSq > maxDiaSq {
			maxDiaSq = diaSq
			maxArea = area
		} else if diaSq == maxDiaSq {
			if area > maxArea {
				maxArea = area
			}
		}
	}
	return maxArea
}

func main() {
	tests := []struct {
		dimensions [][]int
		expected   int
	}{
		{[][]int{{9, 3}, {8, 6}}, 48},
		{[][]int{{3, 4}, {4, 3}}, 12},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.expected, areaOfMaxDiagonal(test.dimensions), index)
	}
}
