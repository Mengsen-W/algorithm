/*
 * @Date: 2023-06-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-28
 * @FilePath: /algorithm/golang/1681_minimum_incompatibility/minimum_incompatibility.go
 */

// Package main ...
package main

import (
	"math"
	"math/bits"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumIncompatibility(nums []int, k int) int {
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
	n := len(nums)
	group := n / k
	inf := math.MaxInt32
	dp := make([]int, 1<<n)
	for i := range dp {
		dp[i] = inf
	}
	dp[0] = 0
	values := make(map[int]int)

	for mask := 1; mask < (1 << n); mask++ {
		if bits.OnesCount(uint(mask)) != group {
			continue
		}
		minVal := 20
		maxVal := 0
		cur := make(map[int]bool)
		for i := 0; i < n; i++ {
			if mask&(1<<i) != 0 {
				if cur[nums[i]] {
					break
				}
				cur[nums[i]] = true
				minVal = min(minVal, nums[i])
				maxVal = max(maxVal, nums[i])
			}
		}
		if len(cur) == group {
			values[mask] = maxVal - minVal
		}
	}

	for mask := 0; mask < (1 << n); mask++ {
		if dp[mask] == inf {
			continue
		}
		seen := make(map[int]int)
		for i := 0; i < n; i++ {
			if (mask & (1 << i)) == 0 {
				seen[nums[i]] = i
			}
		}
		if len(seen) < group {
			continue
		}
		sub := 0
		for _, v := range seen {
			sub |= (1 << v)
		}
		nxt := sub
		for nxt > 0 {
			if val, ok := values[nxt]; ok {
				dp[mask|nxt] = min(dp[mask|nxt], dp[mask]+val)
			}
			nxt = (nxt - 1) & sub
		}
	}
	if dp[(1<<n)-1] < inf {
		return dp[(1<<n)-1]
	}
	return -1
}

func main() {
	testMap := []struct {
		nums []int
		k    int
		ans  int
	}{
		{[]int{1, 2, 1, 4}, 2, 4},
		{[]int{6, 3, 8, 1, 3, 1, 2, 2}, 4, 6},
		{[]int{5, 3, 3, 6, 3, 3}, 3, -1},
	}

	for _, item := range testMap {
		assert.Equal(&testing.T{}, minimumIncompatibility(item.nums, item.k), item.ans)
	}
}
