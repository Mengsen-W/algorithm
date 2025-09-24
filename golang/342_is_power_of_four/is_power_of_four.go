// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func isPowerOfFour(n int) bool {
	return n > 0 && n&(n-1) == 0 && n%3 == 1
}

func main() {
	tests := []struct {
		n   int
		ans bool
	}{
		{16, true},
		{5, false},
		{1, true},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, isPowerOfFour(test.n), index)
	}
}
