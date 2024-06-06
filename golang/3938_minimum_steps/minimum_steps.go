// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumSteps(s string) int64 {
	ans := int64(0)
	sum := 0
	for i := 0; i < len(s); i++ {
		if s[i] == '1' {
			sum++
		} else {
			ans += int64(sum)
		}
	}
	return ans
}

func main() {
	tests := []struct {
		s   string
		ans int64
	}{
		{"101", 1},
		{"100", 2},
		{"0111", 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimumSteps(test.s), index)
	}
}
