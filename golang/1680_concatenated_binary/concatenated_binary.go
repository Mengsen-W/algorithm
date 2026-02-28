// Package main ...
package main

import (
	"math/bits"
	"testing"

	"github.com/stretchr/testify/assert"
)

func concatenatedBinary(n int) (ans int) {
	for i := 1; i <= n; i++ {
		ans = (ans<<bits.Len(uint(i)) | i) % (1e9 + 7)
	}
	return
}

func main() {
	tests := []struct {
		n   int
		ans int
	}{
		{1, 1},
		{3, 27},
		{12, 505379714},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, concatenatedBinary(test.n), index)
	}
}
