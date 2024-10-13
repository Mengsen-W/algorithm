// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func twoEggDrop(n int) int {
	return int(math.Ceil((-1 + math.Sqrt(1+8*float64(n))) / 2))
}

func main() {
	tests := []struct {
		n   int
		ans int
	}{
		{2, 2},
		{100, 14},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, twoEggDrop(test.n), index)
	}
}
