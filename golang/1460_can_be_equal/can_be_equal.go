/*
 * @Date: 2022-08-24
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-24
 * @FilePath: /algorithm/1460_can_be_equal/can_be_equal.go
 */

package main

import "sort"

func canBeEqual(target, arr []int) bool {
	sort.Ints(target)
	sort.Ints(arr)
	for i, x := range target {
		if x != arr[i] {
			return false
		}
	}
	return true
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		target := []int{1, 2, 3, 4}
		arr := []int{2, 4, 1, 3}
		assert(canBeEqual(target, arr))
	}
	{
		target := []int{7}
		arr := []int{7}
		assert(canBeEqual(target, arr))
	}

	{
		target := []int{3, 7, 9}
		arr := []int{3, 7, 11}
		assert(!canBeEqual(target, arr))
	}
}
