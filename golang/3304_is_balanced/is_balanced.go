// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func isBalanced(num string) bool {
	diff := 0
	sign := 1
	for _, c := range num {
		d := int(c - '0')
		diff += d * sign
		sign = -sign
	}
	return diff == 0
}

func main() {
	tests := []struct {
		num string
		ans bool
	}{
		{"1234", false},
		{"24123", true},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, isBalanced(test.num), index)
	}
}
