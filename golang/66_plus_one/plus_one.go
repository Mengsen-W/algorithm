// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func plusOne(digits []int) []int {
	for i := len(digits) - 1; i != -1; i-- {
		digits[i]++
		if digits[i]/10 == 0 {
			return digits
		}
		digits[i] = digits[i] % 10
	}
	return append([]int{1}, digits...)
}

func main() {
	tests := []struct {
		digits []int
		ans    []int
	}{
		{[]int{1, 2, 3}, []int{1, 2, 4}},
		{[]int{1, 2, 9}, []int{1, 3, 0}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, plusOne(test.digits), index)
	}
}
