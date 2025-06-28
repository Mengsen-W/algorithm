// Package main ...
package main

import (
	"sort"

	"github.com/stretchr/testify/assert"
)

func maxSubsequence(nums []int, k int) []int {
	n := len(nums)
	vals := make([][]int, n) // 辅助数组
	for i := 0; i < n; i++ {
		vals[i] = []int{i, nums[i]}
	}
	// 按照数值降序排序
	sort.Slice(vals, func(i, j int) bool {
		return vals[i][1] > vals[j][1]
	})
	// 取前 k 个并按照下标升序排序
	sort.Slice(vals[:k], func(i, j int) bool {
		return vals[i][0] < vals[j][0]
	})
	res := make([]int, k) // 目标子序列
	for i := 0; i < k; i++ {
		res[i] = vals[i][1]
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		k    int
		ans  []int
	}{
		{[]int{2, 1, 3, 3}, 2, []int{3, 3}},
		{[]int{-1, -2, 3, 4}, 3, []int{-1, 3, 4}},
		{[]int{3, 4, 3, 3}, 2, []int{3, 4}},
	}

	for index, test := range tests {
		assert.Equal(nil, test.ans, maxSubsequence(test.nums, test.k), "case %d", index)
	}
}
