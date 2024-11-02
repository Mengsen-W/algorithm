// Package main ...
package main

import (
	"math/bits"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minChanges(n int, k int) int {
	if (n & k) == k {
		return bits.OnesCount(uint(n ^ k))
	}
	return -1
}

func main() {
	tests := []struct {
		n   int
		k   int
		ans int
	}{
		{13, 4, 2},
		{21, 21, 0},
		{14, 13, -1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minChanges(test.n, test.k), index)
	}
}
