/*
 * @Date: 2023-06-04
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-04
 * @FilePath: /algorithm/golang/2465_distinct_averages/distinct_averages.go
 */

// Package main ...
package main

import "sort"

func distinctAverages(nums []int) int {
	sort.Ints(nums)
	seen := make(map[int]bool)
	for i, j := 0, len(nums)-1; i < j; i, j = i+1, j-1 {
		seen[nums[i]+nums[j]] = true
	}
	return len(seen)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		nums := []int{4, 1, 4, 0, 3, 5}
		ans := 2
		assert(distinctAverages(nums) == ans)
	}

	{
		nums := []int{1, 100}
		ans := 1
		assert(distinctAverages(nums) == ans)
	}
}
