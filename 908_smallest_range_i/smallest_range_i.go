/*
 * @Date: 2022-04-30 08:16:44
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-30 08:27:27
 * @FilePath: /algorithm/908_smallest_range_i/smallest_range_i.go
 */

package main

func smallestRangeI(nums []int, k int) int {
	minNum, maxNum := nums[0], nums[0]
	for _, num := range nums[1:] {
		if num < minNum {
			minNum = num
		} else if num > maxNum {
			maxNum = num
		}
	}
	ans := maxNum - minNum - 2*k
	if ans > 0 {
		return ans
	}
	return 0
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(smallestRangeI([]int{1}, 0) == 0)
	assert(smallestRangeI([]int{0, 10}, 2) == 6)
	assert(smallestRangeI([]int{1, 3, 6}, 3) == 0)
}
