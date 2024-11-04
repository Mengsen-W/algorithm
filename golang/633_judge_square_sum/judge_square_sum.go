package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func judgeSquareSum(c int) bool {
	for base := 2; base*base <= c; base++ {
		if c%base > 0 {
			continue
		}

		exp := 0
		for ; c%base == 0; c /= base {
			exp++
		}

		if base%4 == 3 && exp%2 != 0 {
			return false
		}
	}

	return c%4 != 3
}

func main() {
	tests := []struct {
		c   int
		ans bool
	}{
		{5, true},
		{3, false},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, judgeSquareSum(test.c), index)
	}
}
