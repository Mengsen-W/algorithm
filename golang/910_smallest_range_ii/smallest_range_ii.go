// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func smallestRangeII(nums []int, k int) int {
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}

	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	sort.Ints(nums)
	mi, ma := nums[0], nums[len(nums)-1]
	res := ma - mi
	for i := 0; i < len(nums)-1; i++ {
		a, b := nums[i], nums[i+1]
		res = min(res, max(ma-k, a+k)-min(mi+k, b-k))
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		k    int
		ans  int
	}{
		{[]int{1}, 0, 0},
		{[]int{0, 10}, 2, 6},
		{[]int{1, 3, 6}, 3, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, smallestRangeII(test.nums, test.k), index)
	}
}
