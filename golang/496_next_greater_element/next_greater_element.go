/*
 * @Date: 2021-10-26 01:01:55
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-26 01:08:55
 */

package main

import "reflect"

func nextGreaterElement(nums1, nums2 []int) []int {
	mp := map[int]int{}
	stack := []int{}
	for i := len(nums2) - 1; i >= 0; i-- {
		num := nums2[i]
		for len(stack) > 0 && num >= stack[len(stack)-1] {
			stack = stack[:len(stack)-1]
		}
		if len(stack) > 0 {
			mp[num] = stack[len(stack)-1]
		} else {
			mp[num] = -1
		}
		stack = append(stack, num)
	}
	res := make([]int, len(nums1))
	for i, num := range nums1 {
		res[i] = mp[num]
	}
	return res
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	{
		nums1 := []int{4, 1, 2}
		nums2 := []int{1, 3, 4, 2}
		ans := []int{-1, 3, -1}
		assert(nextGreaterElement(nums1, nums2), ans)
	}
	{
		nums1 := []int{2, 4}
		nums2 := []int{1, 2, 3, 4}
		ans := []int{3, -1}
		assert(nextGreaterElement(nums1, nums2), ans)
	}
}
