// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countPrefixes(words []string, s string) int {
	res := 0
	for _, word := range words {
		if len(s) >= len(word) && s[:len(word)] == word {
			res++
		}
	}
	return res
}

func main() {
	tests := []struct {
		words []string
		s     string
		ans   int
	}{
		{[]string{"a", "b", "c", "ab", "bc", "abc"}, "abc", 3},
		{[]string{"a", "a"}, "aa", 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countPrefixes(test.words, test.s), index)
	}
}
