/*
 * @Date: 2021-09-07 17:26:57
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-07 17:30:32
 */

package main

func search(nums []int, target int) int {
	low, high := 0, len(nums)-1
	for low <= high {
		mid := (high-low)/2 + low
		num := nums[mid]
		if num == target {
			return mid
		} else if num > target {
			high = mid - 1
		} else {
			low = mid + 1
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
	assert(search([]int{-1, 0, 3, 5, 9, 12}, 9) == 4)
	assert(search([]int{-1, 0, 3, 5, 9, 12}, 2) == -1)
}
