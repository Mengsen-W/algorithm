// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func largestPerimeter(nums []int) int {
	sort.Ints(nums)
	for i := len(nums) - 1; i >= 2; i-- {
		if nums[i-2]+nums[i-1] > nums[i] {
			return nums[i-2] + nums[i-1] + nums[i]
		}
	}
	return 0
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{2, 1, 2}, 5},
		{[]int{1, 2, 1, 10}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, largestPerimeter(test.nums), "case %d", index)
	}
}
