/*
 * @Date: 2021-07-20 14:13:19
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-20 14:19:04
 */

package main

import "sort"

func minPairSum(nums []int) (ans int) {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	sort.Ints(nums)
	n := len(nums)
	for i, val := range nums[:n/2] {
		ans = max(ans, val+nums[n-1-i])
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		nums := []int{3, 5, 2, 3}
		ans := 7
		assert(minPairSum(nums) == ans)
	}
	{
		nums := []int{3, 5, 4, 2, 4, 6}
		ans := 8
		assert(minPairSum(nums) == ans)
	}
}
