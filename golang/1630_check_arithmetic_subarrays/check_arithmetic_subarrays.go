/*
 * @Date: 2023-03-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-23
 * @FilePath: /algorithm/golang/1630_check_arithmetic_subarrays/check_arithmetic_subarrays.go
 */

// Package main ...
package main

import "reflect"

func checkArithmeticSubarrays(nums []int, l []int, r []int) []bool {
	ans := make([]bool, len(l))
	for i := range l {
		ans[i] = isArithmetic(nums[l[i] : r[i]+1])
	}
	return ans
}

func isArithmetic(nums []int) bool {
	n := len(nums)
	max, min := nums[0], nums[0]
	for _, v := range nums {
		if v > max {
			max = v
		}
		if v < min {
			min = v
		}
	}
	if max == min {
		return true
	}
	d := (max - min) / (n - 1)
	if (max - min) != d*(n-1) {
		return false
	}
	m := make(map[int]struct{})
	for _, v := range nums {
		m[v] = struct{}{}
	}
	for n--; n > 0; n-- {
		min += d
		if _, ok := m[min]; !ok {
			return false
		}
	}
	return true
}

func main() {
	assert := func(a, b []bool) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		nums := []int{4, 6, 5, 9, 3, 7}
		l := []int{0, 0, 2}
		r := []int{2, 3, 5}
		ans := []bool{true, false, true}
		assert(checkArithmeticSubarrays(nums, l, r), ans)
	}

	{
		nums := []int{-12, -9, -3, -12, -6, 15, 20, -25, -20, -15, -10}
		l := []int{0, 1, 6, 4, 8, 7}
		r := []int{4, 4, 9, 7, 9, 10}
		ans := []bool{false, true, false, false, true, true}
		assert(checkArithmeticSubarrays(nums, l, r), ans)
	}
}
