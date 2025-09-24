// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func check(x int, tops, bottoms []int, n int) int {
	rotationsA, rotationsB := 0, 0
	for i := 0; i < n; i++ {
		if tops[i] != x && bottoms[i] != x {
			return -1
		} else if tops[i] != x {
			rotationsA++
		} else if bottoms[i] != x {
			rotationsB++
		}
	}
	return min(rotationsA, rotationsB)
}

func minDominoRotations(tops []int, bottoms []int) int {
	n := len(tops)
	rotations := check(tops[0], tops, bottoms, n)
	if rotations != -1 || tops[0] == bottoms[0] {
		return rotations
	}
	return check(bottoms[0], tops, bottoms, n)
}

func main() {
	tests := []struct {
		a   []int
		b   []int
		ans int
	}{
		{[]int{2, 1, 2, 4, 2, 2}, []int{5, 2, 6, 2, 3, 2}, 2},
		{[]int{3, 5, 1, 2, 3}, []int{3, 6, 3, 3, 4}, -1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minDominoRotations(test.a, test.b), index)
	}
}
