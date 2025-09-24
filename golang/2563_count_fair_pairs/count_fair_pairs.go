// Package main ...
package main

import (
	"slices"
	"testing"

	"github.com/stretchr/testify/assert"
)

func countFairPairs(nums []int, lower, upper int) (ans int64) {
	slices.Sort(nums)
	left, right := len(nums), len(nums)
	for j, x := range nums {
		for right > 0 && nums[right-1] > upper-x {
			right--
		}
		for left > 0 && nums[left-1] >= lower-x {
			left--
		}
		ans += int64(min(right, j) - min(left, j))
	}
	return
}

func main() {
	tests := []struct {
		nums  []int
		lower int
		upper int
		ans   int64
	}{
		{[]int{0, 1, 7, 4, 4, 5}, 3, 6, 6},
		{[]int{1, 7, 9, 2, 5}, 11, 11, 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countFairPairs(test.nums, test.lower, test.upper), index)
	}
}
