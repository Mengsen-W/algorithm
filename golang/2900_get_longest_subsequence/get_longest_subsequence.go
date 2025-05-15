// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func getLongestSubsequence(words []string, groups []int) []string {
	var ans []string
	n := len(words)
	for i := 0; i < n; i++ {
		if i == 0 || groups[i] != groups[i-1] {
			ans = append(ans, words[i])
		}
	}
	return ans
}

func main() {
	tests := []struct {
		words  []string
		groups []int
		ans    []string
	}{
		{[]string{"e", "a", "b"}, []int{0, 0, 1}, []string{"e", "b"}},
		{[]string{"a", "b", "c", "d"}, []int{1, 0, 1, 1}, []string{"a", "b", "c"}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, getLongestSubsequence(test.words, test.groups), "testcase %d", index)
	}
}
