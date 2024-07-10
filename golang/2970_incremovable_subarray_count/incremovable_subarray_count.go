// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func incremovableSubarrayCount(nums []int) int {
	n := len(nums)
	res := 0
	l := 1
	for l < n && nums[l-1] < nums[l] {
		l++
	}

	res += l
	if l < n {
		res += 1
	}
	for r := n - 2; r >= 0; r-- {
		for l > 0 && nums[l-1] >= nums[r+1] {
			l--
		}
		res += l
		if l <= r {
			res += 1
		}
		if nums[r] >= nums[r+1] {
			break
		}
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 2, 3, 4}, 10},
		{[]int{6, 5, 7, 8}, 7},
		{[]int{8, 7, 6, 6}, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, incremovableSubarrayCount(test.nums), index)
	}
}
