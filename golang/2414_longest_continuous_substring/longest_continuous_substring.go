// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func longestContinuousSubstring(s string) int {
	res, cur := 1, 1
	for i := 1; i < len(s); i++ {
		if s[i] == s[i-1]+byte(1) {
			cur++
		} else {
			cur = 1
		}
		res = max(res, cur)
	}
	return res
}

func main() {
	tests := []struct {
		s   string
		ans int
	}{
		{"abacaba", 2},
		{"abcde", 5},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, longestContinuousSubstring(test.s), index)
	}
}
