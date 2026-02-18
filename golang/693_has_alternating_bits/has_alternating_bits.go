// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func hasAlternatingBits(n int) bool {
	a := n ^ n>>1
	return a&(a+1) == 0
}

func main() {
	tests := []struct {
		n   int
		ans bool
	}{
		{5, true},
		{7, false},
		{11, false},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, hasAlternatingBits(test.n), index)
	}
}
