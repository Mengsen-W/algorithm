// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func incremovableSubarrayCount(nums []int) int64 {
	var ans int64 = 0
	length := len(nums)
	l := 0
	for l < length-1 {
		if nums[l] >= nums[l+1] {
			break
		}
		l++
	}

	if l == length-1 {
		return int64(length) * int64(length+1) / 2
	}
	ans += int64(l + 2)
	for r := length - 1; r > 0; r-- {
		if r < length-1 && nums[r] >= nums[r+1] {
			break
		}
		for l >= 0 && nums[l] >= nums[r] {
			l--
		}
		ans += int64(l + 2)
	}

	return ans
}

func main() {
	tests := []struct {
		nums []int
		ans  int64
	}{
		{[]int{1, 2, 3, 4}, 10},
		{[]int{6, 5, 7, 8}, 7},
		{[]int{8, 7, 6, 6}, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, incremovableSubarrayCount(test.nums), index)
	}
}
