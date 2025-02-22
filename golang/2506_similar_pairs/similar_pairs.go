// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func similarPairs(words []string) int {
	res := 0
	cnt := make(map[int]int)
	for _, word := range words {
		state := 0
		for _, c := range word {
			state |= 1 << (c - 'a')
		}
		res += cnt[state]
		cnt[state]++
	}
	return res
}

func main() {
	tests := []struct {
		words []string
		ans   int
	}{
		{[]string{"aba", "aabb", "abcd", "bac", "aabc"}, 2},
		{[]string{"aabb", "ab", "ba"}, 3},
		{[]string{"nba", "cba", "dba"}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, similarPairs(test.words), index)
	}
}
