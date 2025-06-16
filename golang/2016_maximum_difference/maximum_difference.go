// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximumDifference(nums []int) int {
	ans := -1
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}
	for i, preMin := 1, nums[0]; i < len(nums); i++ {
		if nums[i] > preMin {
			ans = max(ans, nums[i]-preMin)
		} else {
			preMin = nums[i]
		}
	}
	return ans
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{7, 1, 5, 4}, 4},
		{[]int{9, 4, 3, 2}, -1},
		{[]int{1, 5, 2, 10}, 9},
	}

	for index, test := range tests {
		assert.Equal(&testing.B{}, test.ans, maximumDifference(test.nums), index)
	}
}
