/*
 * @Date: 2023-12-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-22
 * @FilePath: /algorithm/golang/1671_minimum_mountain_removals/minimum_mountain_removals.go
 */

// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumMountainRemovals(nums []int) int {
	reversed := func(nums []int) []int {
		n := len(nums)
		ans := make([]int, n)
		for i := 0; i < n; i++ {
			ans[i] = nums[n-1-i]
		}
		return ans
	}

	getLISArray := func(nums []int) []int {
		n := len(nums)
		dp := make([]int, n)
		var seq []int
		for i := 0; i < n; i++ {
			it := sort.SearchInts(seq, nums[i])
			if it == len(seq) {
				seq = append(seq, nums[i])
				dp[i] = len(seq)
			} else {
				seq[it] = nums[i]
				dp[i] = it + 1
			}
		}
		return dp
	}

	n := len(nums)
	pre := getLISArray(nums)
	suf := getLISArray(reversed(nums))
	suf = reversed(suf)

	ans := 0
	for i := 0; i < n; i++ {
		if pre[i] > 1 && suf[i] > 1 {
			ans = max(ans, pre[i]+suf[i]-1)
		}
	}
	return n - ans
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 3, 1}, 0},
		{[]int{2, 1, 1, 5, 6, 2, 3, 1}, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimumMountainRemovals(test.nums), index)
	}
}
