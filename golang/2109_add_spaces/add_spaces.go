// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func addSpaces(s string, spaces []int) string {
	ans := make([]byte, 0, len(s)+len(spaces))
	j := 0
	for i, c := range s {
		if j < len(spaces) && spaces[j] == i {
			ans = append(ans, ' ')
			j++
		}
		ans = append(ans, byte(c))
	}
	return string(ans)
}

func main() {
	tests := []struct {
		s      string
		spaces []int
		ans    string
	}{
		{"LeetcodeHelpsMeLearn", []int{8, 13, 15}, "Leetcode Helps Me Learn"},
		{"icodeinpython", []int{1, 5, 7, 9}, "i code in py thon"},
		{"spacing", []int{0, 1, 2, 3, 4, 5, 6}, " s p a c i n g"},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, addSpaces(test.s, test.spaces), index)
	}
}
