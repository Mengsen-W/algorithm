/*
 * @Date: 2023-06-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-05
 * @FilePath: /algorithm/golang/2460_apply_operations/apply_operations.go
 */

// Package main ...
package main

import "reflect"

func applyOperations(nums []int) []int {
	n := len(nums)
	j := 0
	for i := 0; i < n; i++ {
		if i+1 < n && nums[i] == nums[i+1] {
			nums[i] *= 2
			nums[i+1] = 0
		}
		if nums[i] != 0 {
			nums[i], nums[j] = nums[j], nums[i]
			j++
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

	{
		nums := []int{1, 2, 2, 1, 1, 0}
		ans := []int{1, 4, 2, 0, 0, 0}
		assert(applyOperations(nums), ans)
	}

	{
		nums := []int{0, 1}
		ans := []int{1, 0}
		assert(applyOperations(nums), ans)
	}
}
