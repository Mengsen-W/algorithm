// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countBinarySubstrings(s string) int {
	var ptr, last, ans int
	n := len(s)
	for ptr < n {
		c := s[ptr]
		count := 0
		for ptr < n && s[ptr] == c {
			ptr++
			count++
		}
		ans += min(count, last)
		last = count
	}

	return ans
}

func main() {
	tests := []struct {
		s   string
		ans int
	}{
		{"00110011", 6},
		{"10101", 4},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, countBinarySubstrings(test.s), test.ans, index)
	}
}
