// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func evenOddBit(n int) []int {
	res := []int{0, 0}
	i := 0
	for n > 0 {
		res[i] += n & 1
		n >>= 1
		i ^= 1
	}
	return res
}

func main() {
	tests := []struct {
		n   int
		ans []int
	}{
		{17, []int{2, 0}},
		{2, []int{0, 1}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, evenOddBit(test.n), index)
	}
}
