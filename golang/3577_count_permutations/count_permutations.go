// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countPermutations(complexity []int) int {
	n := len(complexity)
	for i := 1; i < n; i++ {
		if complexity[i] <= complexity[0] {
			return 0
		}
	}

	ans := 1
	mod := 1000000007
	for i := 2; i < n; i++ {
		ans = ans * i % mod
	}
	return ans
}

func main() {
	tests := []struct {
		complexity []int
		ans        int
	}{
		{[]int{1, 2, 3}, 2},
		{[]int{3, 3, 3, 4, 4, 4}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countPermutations(test.complexity), index)
	}
}
