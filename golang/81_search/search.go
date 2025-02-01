// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func search(nums []int, target int) bool {
	n := len(nums)
	if n == 0 {
		return false
	}
	if n == 1 {
		return nums[0] == target
	}
	l, r := 0, n-1
	for l <= r {
		mid := (l + r) / 2
		if nums[mid] == target {
			return true
		}
		if nums[l] == nums[mid] && nums[mid] == nums[r] {
			l++
			r--
		} else if nums[l] <= nums[mid] {
			if nums[l] <= target && target < nums[mid] {
				r = mid - 1
			} else {
				l = mid + 1
			}
		} else {
			if nums[mid] < target && target <= nums[n-1] {
				l = mid + 1
			} else {
				r = mid - 1
			}
		}
	}
	return false
}

func main() {
	tests := []struct {
		nums   []int
		target int
		ans    bool
	}{
		{[]int{2, 5, 6, 0, 0, 1, 2}, 0, true},
		{[]int{2, 5, 6, 0, 0, 1, 2}, 3, false},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, search(test.nums, test.target), index)
	}
}
