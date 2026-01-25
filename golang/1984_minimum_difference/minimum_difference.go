// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumDifference(nums []int, k int) (ret int) {
	sort.Ints(nums)
	ret = nums[k-1] - nums[0]
	for i := k; i < len(nums); i++ {
		if ret > nums[i]-nums[i-k+1] {
			ret = nums[i] - nums[i-k+1]
		}
	}
	return
}

func main() {
	tests := []struct {
		nums []int
		k    int
		ans  int
	}{
		{[]int{90}, 1, 0},
		{[]int{9, 4, 7, 1}, 2, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimumDifference(test.nums, test.k), index)
	}
}
