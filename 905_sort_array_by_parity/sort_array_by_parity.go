/*
 * @Date: 2022-04-28 09:24:28
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-28 09:32:52
 * @FilePath: /algorithm/905_sort_array_by_parity/sort_array_by_parity.go
 */

package main

import "reflect"

func sortArrayByParity(nums []int) []int {
	left, right := 0, len(nums)-1
	for left < right {
		for left < right && nums[left]%2 == 0 {
			left++
		}
		for left < right && nums[right]%2 == 1 {
			right--
		}
		if left < right {
			nums[left], nums[right] = nums[right], nums[left]
			left++
			right--
		}
	}
	return nums
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	assert(sortArrayByParity([]int{3, 1, 2, 4}), []int{4, 2, 1, 3})
	assert(sortArrayByParity([]int{0}), []int{0})
}
