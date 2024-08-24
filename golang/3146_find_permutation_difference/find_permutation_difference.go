// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func findPermutationDifference(s string, t string) int {
	char2index := make(map[rune]int)
	for i, c := range s {
		char2index[c] = i
	}
	sum := 0
	for i, c := range t {
		sum += int(math.Abs(float64(i - char2index[c])))
	}
	return sum
}

func main() {
	tests := []struct {
		s   string
		t   string
		ans int
	}{
		{"abc", "bac", 2},
		{"abcde", "edbac", 12},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, findPermutationDifference(test.s, test.t), index)
	}
}
