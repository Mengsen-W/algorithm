// Package main...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func doesValidArrayExist(derived []int) bool {
	xor := 0
	for _, x := range derived {
		xor ^= x
	}
	return xor == 0
}

func main() {
	tests := []struct {
		derived []int
		ans     bool
	}{
		{[]int{1, 1, 0}, true},
		{[]int{1, 1}, true},
		{[]int{1, 0}, false},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, doesValidArrayExist(test.derived), index)
	}
}
