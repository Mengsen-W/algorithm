// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func numberOfArrays(differences []int, lower int, upper int) int {
	min := func(x, y int) int {
		if x > y {
			return y
		}
		return x
	}

	max := func(x, y int) int {
		if x > y {
			return x
		}
		return y
	}
	var x, y, cur int
	for _, d := range differences {
		cur += d
		x, y = min(x, cur), max(y, cur)
		if y-x > upper-lower {
			return 0
		}
	}
	return (upper - lower) - (y - x) + 1
}

func main() {
	tests := []struct {
		differences []int
		lower       int
		upper       int
		ans         int
	}{
		{[]int{1, -3, 4}, 1, 6, 2},
		{[]int{3, -4, 5, 1, -2}, -4, 5, 4},
		{[]int{4, -7, 2}, 3, 6, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, numberOfArrays(test.differences, test.lower, test.upper), index)
	}
}
