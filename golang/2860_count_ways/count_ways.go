// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func countWays(nums []int) int {
	n := len(nums)
	res := 0
	sort.Ints(nums)
	for k := 0; k <= n; k++ {
		// 前 k 个元素的最大值是否小于 k
		if k > 0 && nums[k-1] >= k {
			continue
		}
		// 后 n - k 个元素的最小值是否大于 k
		if k < n && nums[k] <= k {
			continue
		}
		res++
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 1}, 2},
		{[]int{6, 0, 3, 3, 6, 7, 2, 7}, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countWays(test.nums), index)
	}
}
