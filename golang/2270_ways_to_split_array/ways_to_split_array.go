// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func waysToSplitArray(nums []int) int {
	n := len(nums)
	left, right := int64(0), int64(0)
	for _, num := range nums {
		right += int64(num)
	}
	ans := 0
	for i := 0; i < n-1; i++ {
		left += int64(nums[i])
		right -= int64(nums[i])
		if left >= right {
			ans++
		}
	}
	return ans
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{10, 4, -8, 7}, 2},
		{[]int{2, 3, 1, 0}, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, waysToSplitArray(test.nums), index)
	}
}
