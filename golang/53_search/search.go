/*
 * @Date: 2021-07-16 09:44:19
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-16 10:48:04
 */

package main

import "sort"

func search(nums []int, target int) int {
	leftmost := sort.SearchInts(nums, target)
	if leftmost == len(nums) || nums[leftmost] != target {
		return 0
	}
	rightmost := sort.SearchInts(nums, target+1) - 1
	return rightmost - leftmost + 1
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		nums := []int{5, 7, 7, 8, 8, 10}
		target := 8
		assert(search(nums, target) == 2)
	}
	{
		nums := []int{5, 7, 7, 8, 8, 10}
		target := 6
		assert(search(nums, target) == 0)
	}
}
