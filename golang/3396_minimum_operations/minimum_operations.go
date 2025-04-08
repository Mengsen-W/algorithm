// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumOperations(nums []int) int {
	seen := make([]bool, 128)
	for i := len(nums) - 1; i >= 0; i-- {
		if seen[nums[i]] {
			return i/3 + 1
		}
		seen[nums[i]] = true
	}
	return 0
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 2, 3, 4, 2, 3, 3, 5, 7}, 2},
		{[]int{4, 5, 6, 4, 4}, 2},
		{[]int{6, 7, 8, 9}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimumOperations(test.nums), index)
	}
}
