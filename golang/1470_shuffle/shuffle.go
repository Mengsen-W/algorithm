/*
 * @Date: 2022-08-29
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-29
 * @FilePath: /algorithm/1470_shuffle/shuffle.go
 */

package main

import "reflect"

func shuffle(nums []int, n int) []int {
	ans := make([]int, n*2)
	for i, num := range nums[:n] {
		ans[2*i] = num
		ans[2*i+1] = nums[n+i]
	}
	return ans
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	{
		nums := []int{2, 5, 1, 3, 4, 7}
		n := 3
		ans := []int{2, 3, 5, 4, 1, 7}
		assert(shuffle(nums, n), ans)
	}

	{
		nums := []int{1, 2, 3, 4, 4, 3, 2, 1}
		n := 4
		ans := []int{1, 4, 2, 3, 3, 2, 4, 1}
		assert(shuffle(nums, n), ans)
	}

	{
		nums := []int{1, 1, 2, 2}
		n := 2
		ans := []int{1, 2, 1, 2}
		assert(shuffle(nums, n), ans)
	}
}
