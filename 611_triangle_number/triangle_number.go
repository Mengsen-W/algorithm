/*
 * @Date: 2021-08-04 14:59:28
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-04 15:02:51
 */

package main

import "sort"

func triangleNumber(nums []int) (ans int) {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	n := len(nums)
	sort.Ints(nums)
	for i, v := range nums {
		k := i
		for j := i + 1; j < n; j++ {
			for k+1 < n && nums[k+1] < v+nums[j] {
				k++
			}
			ans += max(k-j, 0)
		}
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	nums := []int{2, 2, 3, 4}
	assert(triangleNumber(nums) == 3)
}
