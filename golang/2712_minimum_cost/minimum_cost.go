// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumCost(s string) int64 {
	n := len(s)
	var res int64 = 0
	for i := 1; i < n; i++ {
		if s[i] != s[i-1] {
			res += int64(min(i, n-i))
		}
	}
	return res
}

func main() {
	tests := []struct {
		s   string
		ans int64
	}{
		{"0011", 2},
		{"010101", 9},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimumCost(test.s), index)
	}
}
