// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func binaryGap(n int) (ans int) {
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}
	for i, last := 0, -1; n > 0; i++ {
		if n&1 == 1 {
			if last != -1 {
				ans = max(ans, i-last)
			}
			last = i
		}
		n >>= 1
	}
	return
}

func main() {
	tests := []struct {
		n   int
		ans int
	}{
		{22, 2},
		{8, 0},
		{5, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, binaryGap(test.n), index)
	}
}
