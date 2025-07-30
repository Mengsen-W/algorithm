// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func longestSubarray(nums []int) int {
	maxValue := nums[0]
	ans, cnt := 1, 1
	for i := 1; i < len(nums); i++ {
		if nums[i] > maxValue {
			ans, cnt = 1, 1
			maxValue = nums[i]
		} else if nums[i] < maxValue {
			cnt = 0
		} else if nums[i] == maxValue {
			cnt++
		}
		ans = max(ans, cnt)
	}
	return ans
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 2, 3, 3, 2, 2}, 2},
		{[]int{1, 2, 3, 4}, 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, longestSubarray(test.nums), index)
	}
}
