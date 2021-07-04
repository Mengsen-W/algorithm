/*
 * @Date: 2021-07-04 10:23:07
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-04 10:36:27
 */

package main

import "reflect"

func findErrorNums(nums []int) []int {
	xor := 0
	for _, v := range nums {
		xor ^= v
	}
	n := len(nums)
	for i := 1; i <= n; i++ {
		xor ^= i
	}
	lowbit := xor & -xor
	num1, num2 := 0, 0
	for _, v := range nums {
		if v&lowbit == 0 {
			num1 ^= v
		} else {
			num2 ^= v
		}
	}
	for i := 1; i <= n; i++ {
		if i&lowbit == 0 {
			num1 ^= i
		} else {
			num2 ^= i
		}
	}
	for _, v := range nums {
		if v == num1 {
			return []int{num1, num2}
		}
	}
	return []int{num2, num1}
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed!")
		}
	}
	{
		nums := []int{1, 2, 2, 4}
		assert(reflect.DeepEqual(findErrorNums(nums), []int{2, 3}))
	}
	{
		nums := []int{1, 1}
		assert(reflect.DeepEqual(findErrorNums(nums), []int{1, 2}))
	}
}
