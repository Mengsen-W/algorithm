// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func reverseStr(s string, k int) string {
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}
	t := []byte(s)
	for i := 0; i < len(s); i += 2 * k {
		sub := t[i:min(i+k, len(s))]
		for j, n := 0, len(sub); j < n/2; j++ {
			sub[j], sub[n-1-j] = sub[n-1-j], sub[j]
		}
	}
	return string(t)
}

func main() {
	tests := []struct {
		s   string
		k   int
		ans string
	}{
		{"abcdefg", 2, "bacdfeg"},
		{"abcd", 2, "bacd"},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, reverseStr(test.s, test.k), index)
	}
}
