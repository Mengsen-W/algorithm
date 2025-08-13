// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func isPowerOfThree(n int) bool {
	for n > 0 && n%3 == 0 {
		n /= 3
	}
	return n == 1
}

func main() {
	tests := []struct {
		n   int
		ans bool
	}{
		{27, true},
		{0, false},
		{9, true},
		{45, false},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, isPowerOfThree(test.n), index)
	}
}
