// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func scoreOfString(s string) int {
	n, score := len(s), 0
	for i := 0; i+1 < n; i++ {
		i1, i2 := int(s[i]), int(s[i+1])
		if i1-i2 < 0 {
			score += i2 - i1
		} else {
			score += i1 - i2
		}
	}
	return score
}

func main() {
	tests := []struct {
		s   string
		ans int
	}{
		{"hello", 13},
		{"zaz", 50},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, scoreOfString(test.s), index)
	}
}
