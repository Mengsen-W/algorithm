// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func isTrionic(nums []int) bool {
	if nums[0] >= nums[1] { // 一开始必须是递增的
		return false
	}
	cnt := 1
	for i := 2; i < len(nums); i++ {
		if nums[i-1] == nums[i] {
			return false
		}
		if (nums[i-2] < nums[i-1]) != (nums[i-1] < nums[i]) {
			cnt++
		}
	}
	return cnt == 3 // 一定是增减增
}

func main() {
	tests := []struct {
		nums []int
		ans  bool
	}{
		{[]int{1, 3, 5, 4, 2, 6}, true},
		{[]int{2, 1, 3}, false},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, isTrionic(test.nums), index)
	}
}
