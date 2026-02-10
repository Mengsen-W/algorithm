// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func longestBalanced(nums []int) int {
	maxLen := 0

	for i := 0; i < len(nums); i++ {
		odd := make(map[int]int)
		even := make(map[int]int)

		for j := i; j < len(nums); j++ {
			if nums[j]&1 == 1 {
				odd[nums[j]]++
			} else {
				even[nums[j]]++
			}

			if len(odd) == len(even) {
				if j-i+1 > maxLen {
					maxLen = j - i + 1
				}
			}
		}
	}

	return maxLen
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{2, 5, 4, 3}, 4},
		{[]int{3, 2, 2, 5, 4}, 5},
		{[]int{1, 2, 3, 2}, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, longestBalanced(test.nums), test.ans, "test %d", index)
	}
}
