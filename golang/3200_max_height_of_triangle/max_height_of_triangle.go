// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxHeightOfTriangle(red int, blue int) int {
	maxHeight := func(x, y int) int {
		odd := 2*int(math.Sqrt(float64(x))) - 1
		even := 2 * int((-1+math.Sqrt(1+4*float64(y)))/2)
		return min(odd, even) + 1
	}

	return max(maxHeight(red, blue), maxHeight(blue, red))
}

func main() {
	tests := []struct {
		red  int
		blue int
		ans  int
	}{
		{2, 4, 3},
		{2, 1, 2},
		{1, 1, 1},
		{10, 1, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxHeightOfTriangle(test.red, test.blue), index)
	}
}
