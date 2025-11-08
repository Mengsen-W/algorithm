// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumOneBitOperations(n int) int {
	ans := 0
	for n != 0 {
		ans ^= n
		n >>= 1
	}
	return ans
}

func main() {
	tests := []struct {
		n   int
		ans int
	}{
		{3, 2},
		{6, 4},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, minimumOneBitOperations(test.n), test.ans, index)
	}
}
