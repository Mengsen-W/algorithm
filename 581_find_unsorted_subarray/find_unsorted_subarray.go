/*
 * @Date: 2021-08-03 15:41:55
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-03 16:43:02
 */

package main

import (
	"math"
	"sort"
)

func findUnsortedSubarray(nums []int) int {
	return findUnsortedSubarray_sort(nums)
	// return findUnsortedSubarray_traverse(nums)
}

func findUnsortedSubarray_sort(nums []int) int {
	if sort.IntsAreSorted(nums) {
		return 0
	}
	numsSorted := append([]int(nil), nums...)
	sort.Ints(numsSorted)
	left, right := 0, len(nums)-1
	for nums[left] == numsSorted[left] {
		left++
	}
	for nums[right] == numsSorted[right] {
		right--
	}
	return right - left + 1
}

func findUnsortedSubarray_traverse(nums []int) int {
	n := len(nums)
	minn, maxn := math.MaxInt64, math.MinInt64
	left, right := -1, -1
	for i, num := range nums {
		if maxn > num {
			right = i
		} else {
			maxn = num
		}
		if minn < nums[n-i-1] {
			left = n - i - 1
		} else {
			minn = nums[n-i-1]
		}
	}
	if right == -1 {
		return 0
	}
	return right - left + 1
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		nums := []int{2, 6, 4, 8, 10, 9, 15}
		ans := 5
		assert(findUnsortedSubarray_sort(nums) == ans)
		assert(findUnsortedSubarray_traverse(nums) == ans)
	}
	{
		nums := []int{1, 2, 3, 4}
		ans := 0
		assert(findUnsortedSubarray_sort(nums) == ans)
		assert(findUnsortedSubarray_traverse(nums) == ans)
	}
	{
		nums := []int{1}
		ans := 0
		assert(findUnsortedSubarray_sort(nums) == ans)
		assert(findUnsortedSubarray_traverse(nums) == ans)
	}
}
