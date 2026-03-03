// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func findKthBit(n int, k int) byte {
	if k == 1 {
		return '0'
	}
	mid := 1 << (n - 1)
	if k == mid {
		return '1'
	} else if k < mid {
		return findKthBit(n-1, k)
	} else {
		k = mid*2 - k
		return invert(findKthBit(n-1, k))
	}
}

func invert(bit byte) byte {
	if bit == '0' {
		return '1'
	}
	return '0'
}

func main() {
	tests := []struct {
		n   int
		k   int
		ans byte
	}{
		{3, 1, '0'},
		{4, 11, '1'},
		{1, 1, '0'},
		{2, 3, '1'},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, findKthBit(test.n, test.k), "test %d", index)
	}
}
