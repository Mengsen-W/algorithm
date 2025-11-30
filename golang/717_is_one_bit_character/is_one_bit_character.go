// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func isOneBitCharacter(bits []int) bool {
	n := len(bits)
	i := n - 2
	for i >= 0 && bits[i] == 1 {
		i--
	}
	return (n-i)%2 == 0
}

func main() {
	tests := []struct {
		bits []int
		ans  bool
	}{
		{bits: []int{1, 0, 0}, ans: true},
		{bits: []int{1, 1, 1, 0}, ans: false},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, isOneBitCharacter(test.bits), index)
	}
}
