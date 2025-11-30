// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxOperations(s string) int {
	countOne := 0
	ans := 0
	i := 0
	for i < len(s) {
		if s[i] == '0' {
			for i+1 < len(s) && s[i+1] == '0' {
				i++
			}
			ans += countOne
		} else {
			countOne++
		}
		i++
	}
	return ans
}

func main() {
	tests := []struct {
		s   string
		ans int
	}{
		{"1001101", 4},
		{"00111", 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxOperations(test.s), index)
	}
}
