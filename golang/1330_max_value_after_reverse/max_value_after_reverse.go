/*
 * @Date: 2023-05-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-12
 * @FilePath: /algorithm/golang/1330_max_value_after_reverse/max_value_after_reverse.go
 */

// Package main ...
package main

import "fmt"

func maxValueAfterReverse(nums []int) int {
	abs := func(a int) int {
		if a < 0 {
			return -a
		}
		return a
	}

	max := func(a, b int) int {
		if a < b {
			return b
		}
		return a
	}

	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}

	value, n := 0, len(nums)
	for i := 0; i < n-1; i++ {
		value += abs(nums[i] - nums[i+1])
	}
	mx1 := 0
	for i := 1; i < n-1; i++ {
		mx1 = max(mx1, abs(nums[0]-nums[i+1])-abs(nums[i]-nums[i+1]))
		mx1 = max(mx1, abs(nums[n-1]-nums[i-1])-abs(nums[i]-nums[i-1]))
	}
	mx2, mn2 := -100000, 100000
	for i := 0; i < n-1; i++ {
		x, y := nums[i], nums[i+1]
		mx2 = max(mx2, min(x, y))
		mn2 = min(mn2, max(x, y))
	}
	return value + max(mx1, 2*(mx2-mn2))
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		nums := []int{2, 3, 1, 5, 4}
		ans := 10
		assert(maxValueAfterReverse(nums) == ans)
	}

	{
		nums := []int{2, 4, 9, 24, 2, 1, 10}
		ans := 68
		assert(maxValueAfterReverse(nums) == ans)
	}
}
