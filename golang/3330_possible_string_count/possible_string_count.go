// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func possibleStringCount(word string) int {
	n, ans := len(word), 1
	for i := 1; i < n; i++ {
		if word[i-1] == word[i] {
			ans++
		}
	}
	return ans
}

func main() {
	tests := []struct {
		word string
		ans  int
	}{
		{"abbcccc", 5},
		{"abcd", 1},
		{"aaaa", 4},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, possibleStringCount(test.word), index)
	}
}
