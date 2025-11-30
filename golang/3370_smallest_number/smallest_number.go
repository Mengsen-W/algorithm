// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func smallestNumber(n int) int {
	x := 1
	for x < n {
		x = x*2 + 1
	}
	return x
}

func main() {
	tests := []struct {
		n   int
		ans int
	}{
		{5, 7},
		{10, 15},
		{3, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, smallestNumber(test.n), index)
	}
}
