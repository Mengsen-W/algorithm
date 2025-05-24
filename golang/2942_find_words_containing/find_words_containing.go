// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func findWordsContaining(words []string, x byte) []int {
	res := []int{}
	n := len(words)
	for i := 0; i < n; i++ {
		for j := 0; j < len(words[i]); j++ {
			if words[i][j] == x {
				res = append(res, i)
				break
			}
		}
	}
	return res
}

func main() {
	tests := []struct {
		words []string
		x     byte
		ans   []int
	}{
		{[]string{"leet", "code"}, 'e', []int{0, 1}},
		{[]string{"abc", "bcd", "aaaa", "cbc"}, 'a', []int{0, 2}},
		{[]string{"abc", "bcd", "aaaa", "cbc"}, 'z', []int{}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, findWordsContaining(test.words, test.x), index)
	}
}
