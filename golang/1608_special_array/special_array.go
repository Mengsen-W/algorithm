/*
 * @Date: 2022-09-12
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-12
 * @FilePath: /algorithm/1608_special_array/special_array.go
 */

package main

import (
	"sort"
)

func specialArray(nums []int) int {
	sort.Sort(sort.Reverse(sort.IntSlice(nums)))
	for i, n := 1, len(nums); i <= n; i++ {
		if nums[i-1] >= i && (i == n || nums[i] < i) {
			return i
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

	// {
	// 	nums := []int{3, 5}
	// 	ans := 2
	// 	assert(specialArray(nums) == ans)
	// }

	// {
	// 	nums := []int{0, 0}
	// 	ans := -1
	// 	assert(specialArray(nums) == ans)
	// }

	{
		nums := []int{0, 4, 3, 0, 4}
		ans := 3
		assert(specialArray(nums) == ans)
	}

	// {
	// 	nums := []int{3, 6, 7, 7, 0}
	// 	ans := -1
	// 	assert(specialArray(nums) == ans)
	// }
}
