/*
 * @Date: 2022-02-26 01:33:40
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-26 01:40:54
 * @FilePath: /algorithm/2016_maximum_difference/maximum_difference.go
 */

package main

func maximumDifference(nums []int) int {
	ans := -1
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}
	for i, preMin := 1, nums[0]; i < len(nums); i++ {
		if nums[i] > preMin {
			ans = max(ans, nums[i]-preMin)
		} else {
			preMin = nums[i]
		}
	}
	return ans
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(maximumDifference([]int{7, 1, 5, 4}) == 4)
	assert(maximumDifference([]int{9, 4, 3, 2}) == -1)
	assert(maximumDifference([]int{1, 5, 2, 10}) == 9)
}
