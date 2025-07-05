// Package main ...
package main

import (
	"math/bits"
	"testing"

	"github.com/stretchr/testify/assert"
)

func kthCharacter(k int64, operations []int) byte {
	ans := 0
	for k != 1 {
		t := bits.Len64(uint64(k)) - 1
		if (1 << t) == k {
			t--
		}
		k -= (1 << t)
		if operations[t] != 0 {
			ans++
		}
	}
	return byte('a' + (ans % 26))
}

func main() {
	tests := []struct {
		k          int64
		operations []int
		ans        byte
	}{
		{5, []int{0, 0, 0}, 'a'},
		{10, []int{0, 1, 0, 1}, 'b'},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, kthCharacter(test.k, test.operations), index)
	}
}
