// Package main ...
package main

import (
	"math/bits"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimizedStringLength(s string) int {
	var mask uint
	for _, c := range s {
		mask |= 1 << (c - 'a')
	}
	return bits.OnesCount(mask)
}

func main() {
	tests := []struct {
		s   string
		ans int
	}{
		{"aaabc", 3},
		{"cbbd", 3},
		{"dddaaa", 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimizedStringLength(test.s), index)
	}
}
