// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func hasAllCodes(s string, k int) bool {
	if len(s) < (1<<k)+k-1 {
		return false
	}

	num := 0
	for i := 0; i < k; i++ {
		num = num << 1
		if s[i] == '1' {
			num |= 1
		}
	}

	exists := make(map[int]bool)
	exists[num] = true
	for i := 1; i+k <= len(s); i++ {
		num = (num-(int(s[i-1]-'0')<<(k-1)))*2 + int(s[i+k-1]-'0')
		exists[num] = true
	}
	return len(exists) == (1 << k)
}

func main() {
	tests := []struct {
		s   string
		k   int
		ans bool
	}{
		{"00110110", 2, true},
		{"0110", 1, true},
		{"0110", 2, false},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, hasAllCodes(test.s, test.k), index)
	}
}
