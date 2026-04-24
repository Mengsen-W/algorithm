// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func furthestDistanceFromOrigin(moves string) int {
	L, R, B := 0, 0, 0
	for _, c := range moves {
		if c == 'L' {
			L++
		} else if c == 'R' {
			R++
		} else {
			B++
		}
	}
	return int(math.Abs(float64(L-R))) + B
}

func main() {
	tests := []struct {
		moves string
		ans   int
	}{
		{"L_RL__R", 3},
		{"_R__LL_", 5},
		{"_______", 7},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, furthestDistanceFromOrigin(test.moves), index)
	}
}
