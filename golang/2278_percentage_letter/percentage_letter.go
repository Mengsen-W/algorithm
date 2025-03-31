// Package main ...
package main

import (
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func percentageLetter(s string, letter byte) int {
	return strings.Count(s, string(letter)) * 100 / len(s)
}

func main() {
	tests := []struct {
		s      string
		letter byte
		ans    int
	}{
		{"foobar", 'o', 33},
		{"jjjj", 'k', 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, percentageLetter(test.s, test.letter), index)
	}
}
