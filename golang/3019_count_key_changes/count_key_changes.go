// Package main ...
package main

import (
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func countKeyChanges(s string) int {
	ans := 0
	for i := 1; i < len(s); i++ {
		if !strings.EqualFold(string(s[i-1]), string(s[i])) {
			ans++
		}
	}
	return ans
}

func main() {
	tests := []struct {
		s   string
		ans int
	}{
		{"aAbBcC", 2},
		{"AaAaAaaA", 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countKeyChanges(test.s), index)
	}
}
