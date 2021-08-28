/*
 * @Date: 2021-08-28 14:51:26
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-28 15:07:03
 */

package main

import (
	"reflect"
)

func runningSum(nums []int) []int {
	n := len(nums)
	for i := 1; i < n; i++ {
		nums[i] += nums[i-1]
	}
	return nums
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	{
		nums := []int{1, 2, 3, 4}
		ans := []int{1, 3, 6, 10}
		assert(runningSum(nums), ans)
	}
	{
		nums := []int{1, 1, 1, 1, 1}
		ans := []int{1, 2, 3, 4, 5}
		assert(runningSum(nums), ans)
	}
	{
		nums := []int{3, 1, 2, 10, 1}
		ans := []int{3, 4, 6, 16, 17}
		assert(runningSum(nums), ans)
	}
}
