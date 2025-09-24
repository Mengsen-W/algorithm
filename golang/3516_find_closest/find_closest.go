// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func findClosest(x int, y int, z int) int {
	dxz := int(math.Abs(float64(x - z)))
	dyz := int(math.Abs(float64(y - z)))
	if dxz < dyz {
		return 1
	} else if dxz > dyz {
		return 2
	}
	return 0
}

func main() {
	tests := []struct {
		x        int
		y        int
		z        int
		expected int
	}{
		{2, 7, 4, 1},
		{2, 5, 6, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.expected, findClosest(test.x, test.y, test.z), index)
	}
}
