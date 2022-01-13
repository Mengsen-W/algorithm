/*
 * @Date: 2022-01-13 01:33:16
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-13 02:25:39
 */

package main

func dominantIndex(nums []int) int {
	max, second, idx := -1, -1, -1
	for i, value := range nums {
		if value > max {
			max, second, idx = value, max, i
		} else if value > second {
			second = value
		}
	}
	if max >= 2*second {
		return idx
	}
	return -1
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(dominantIndex([]int{3, 6, 1, 0}) == 1)
	assert(dominantIndex([]int{1, 2, 3, 4}) == -1)
	assert(dominantIndex([]int{1}) == 0)
}
