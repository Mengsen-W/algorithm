/*
 * @Date: 2023-05-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-13
 * @FilePath: /algorithm/golang/2441_find_max_k/find_max_k.go
 */

// Package main ...
package main

import "sort"

func findMaxK(nums []int) int {
	sort.Ints(nums)
	for i, j := 0, len(nums)-1; i < j; j-- {
		for i < j && nums[i] < -nums[j] {
			i++
		}
		if nums[i] == -nums[j] {
			return nums[j]
		}
	}
	return -1
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		nums := []int{-1, 2, -3, 3}
		ans := 3
		assert(findMaxK(nums) == ans)
	}

	{
		nums := []int{-1, 10, 6, 7, -7, 1}
		ans := 7
		assert(findMaxK(nums) == ans)
	}

	{
		nums := []int{-10, 8, 6, 7, -2, -3}
		ans := -1
		assert(findMaxK(nums) == ans)
	}
}
