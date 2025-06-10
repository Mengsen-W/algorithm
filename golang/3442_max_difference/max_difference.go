// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxDifference(s string) int {
	c := make(map[rune]int)
	for _, ch := range s {
		c[ch]++
	}
	maxOdd, minEven := 1, len(s)
	for _, value := range c {
		if value%2 == 1 {
			maxOdd = max(maxOdd, value)
		} else {
			minEven = min(minEven, value)
		}
	}
	return maxOdd - minEven
}

func main() {
	tests := []struct {
		s   string
		ans int
	}{
		{"aaaaabbc", 3},
		{"abcabcab", 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxDifference(test.s), "case %d", index)
	}
}
