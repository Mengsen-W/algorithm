/*
 * @Date: 2021-10-20 12:16:37
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-20 12:20:49
 */

package main

func minMoves(nums []int) (ans int) {
	min := nums[0]
	for _, num := range nums[1:] {
		if num < min {
			min = num
		}
	}
	for _, num := range nums {
		ans += num - min
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(minMoves([]int{1, 2, 3}) == 3)
	assert(minMoves([]int{1, 1, 1}) == 0)
}
