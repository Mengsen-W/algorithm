// Package main ...
package main

import (
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func doesAliceWin(s string) bool {
	return strings.ContainsAny(s, "aeiou")
}

func main() {
	tests := []struct {
		s   string
		ans bool
	}{
		{"leetcoder", true},
		{"bbcd", false},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, doesAliceWin(test.s), index)
	}
}
