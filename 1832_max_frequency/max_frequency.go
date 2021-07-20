/*
 * @Date: 2021-07-20 14:04:40
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-20 14:09:35
 */

package main

import "sort"

func maxFrequency(nums []int, k int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	sort.Ints(nums)
	ans := 1
	for l, r, total := 0, 1, 0; r < len(nums); r++ {
		total += (nums[r] - nums[r-1]) * (r - l)
		for total > k {
			total -= nums[r] - nums[l]
			l++
		}
		ans = max(ans, r-l+1)
	}
	return ans
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		nums := []int{1, 2, 4}
		k := 5
		ans := 3
		assert(maxFrequency(nums, k) == ans)
	}
	{
		nums := []int{1, 4, 8, 13}
		k := 5
		ans := 2
		assert(maxFrequency(nums, k) == ans)
	}

}
