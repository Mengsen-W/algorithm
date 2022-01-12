/*
 * @Date: 2022-01-12 00:48:22
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-12 01:06:17
 */

package main

import "math"

func increasingTriplet(nums []int) bool {
	n := len(nums)
	if n < 3 {
		return false
	}
	first, second := nums[0], math.MaxInt32
	for i := 1; i < n; i++ {
		num := nums[i]
		if num > second {
			return true
		} else if num > first {
			second = num
		} else {
			first = num
		}
	}
	return false
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(increasingTriplet([]int{1, 2, 3, 4, 5}))
	assert(!increasingTriplet([]int{5, 4, 3, 2, 1}))
	assert(increasingTriplet([]int{2, 1, 5, 0, 4, 6}))
}
