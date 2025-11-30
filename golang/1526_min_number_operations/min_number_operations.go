// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minNumberOperations(target []int) int {
	n := len(target)
	ans := target[0]
	for i := 1; i < n; i++ {
		ans += max(target[i]-target[i-1], 0)
	}
	return ans
}

func main() {
	tests := []struct {
		target []int
		ans    int
	}{
		{[]int{1, 2, 3, 2, 1}, 3},
		{[]int{3, 1, 1, 2}, 4},
		{[]int{3, 1, 5, 4, 2}, 7},
		{[]int{1, 1, 1, 1}, 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minNumberOperations(test.target), "case %d", index)
	}
}
