/*
 * @Date: 2022-02-11 00:14:01
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-11 00:27:37
 * @FilePath: /algorithm/1984_minimum_difference/minimum_difference.go
 */

package main

import "sort"

func minimumDifference(nums []int, k int) (ret int) {
	sort.Ints(nums)
	ret = nums[k-1] - nums[0]
	for i := k; i < len(nums); i++ {
		if ret > nums[i]-nums[i-k+1] {
			ret = nums[i] - nums[i-k+1]
		}
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(minimumDifference([]int{90}, 1) == 0)
	assert(minimumDifference([]int{9, 4, 7, 1}, 2) == 2)
}
