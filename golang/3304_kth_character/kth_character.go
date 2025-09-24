// Package main ...
package main

import (
	"math/bits"
	"testing"

	"github.com/stretchr/testify/assert"
)

func kthCharacter(k int) byte {
	ans := 0
	for k != 1 {
		t := bits.Len(uint(k)) - 1
		if 1<<t == k {
			t--
		}
		k -= 1 << t
		ans++
	}
	return byte('a' + ans)
}

func main() {
	tests := []struct {
		k   int
		ans byte
	}{
		{5, 'b'},
		{10, 'c'},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, kthCharacter(test.k), index)
	}
}
