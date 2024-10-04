// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func nthPersonGetsNthSeat(n int) float64 {
	if n == 1 {
		return 1.0
	}
	return 0.5
}

func main() {
	tests := []struct {
		n   int
		ans float64
	}{
		{1, 1.0},
		{2, 0.5},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, nthPersonGetsNthSeat(test.n), index)
	}
}
