// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func isSubstringPresent(s string) bool {
	h := make([]int, 26)
	for i := 0; i+1 < len(s); i++ {
		x, y := s[i]-'a', s[i+1]-'a'
		h[x] |= (1 << y)
		if (h[y]>>x)&1 != 0 {
			return true
		}
	}
	return false
}

func main() {
	tests := []struct {
		s   string
		ans bool
	}{
		{"leetcode", true},
		{"abcba", true},
		{"abcd", false},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, isSubstringPresent(test.s), index)
	}
}
