// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func edgeScore(edges []int) int {
	n := len(edges)
	points := make([]int64, n)
	for i, x := range edges {
		points[x] += int64(i)
	}
	maxPoints := int64(-1)
	res := -1
	for i := 0; i < n; i++ {
		if points[i] > maxPoints {
			maxPoints = points[i]
			res = i
		}
	}
	return res
}

func main() {
	tests := []struct {
		edges []int
		ans   int
	}{
		{[]int{1, 0, 0, 0, 0, 7, 7, 5}, 7},
		{[]int{2, 0, 0, 2}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, edgeScore(test.edges), index)
	}
}
