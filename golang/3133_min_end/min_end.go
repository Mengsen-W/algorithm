// Package main ...
package main

import (
	"math/bits"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minEnd(n int, x int) int64 {
	bitCount := 128 - bits.LeadingZeros(uint(n)) - bits.LeadingZeros(uint(x))
	res := int64(x)
	m := int64(n) - 1
	j := 0
	for i := 0; i < bitCount; i++ {
		if res&(1<<i) == 0 {
			if m&(1<<j) != 0 {
				res |= 1 << i
			}
			j++
		}
	}
	return res
}

func main() {
	tests := []struct {
		n   int
		x   int
		ans int64
	}{
		{3, 4, 6},
		{2, 7, 15},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minEnd(test.n, test.x), index)
	}
}
