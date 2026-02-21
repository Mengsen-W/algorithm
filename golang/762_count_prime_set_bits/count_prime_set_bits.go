// Package main ...
package main

import (
	"math/bits"
	"testing"

	"github.com/stretchr/testify/assert"
)

func countPrimeSetBits(left, right int) (ans int) {
	for x := left; x <= right; x++ {
		if 1<<bits.OnesCount(uint(x))&665772 != 0 {
			ans++
		}
	}
	return
}

func main() {
	tests := []struct {
		left  int
		right int
		ans   int
	}{
		{6, 10, 4},
		{10, 15, 5},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countPrimeSetBits(test.left, test.right), index)
	}
}
