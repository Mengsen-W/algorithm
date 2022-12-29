/*
 * @Date: 2022-12-29
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-29
 * @FilePath: /algorithm/2023_two_out_of_three/two_out_of_three.go
 */

package main

import "reflect"

func twoOutOfThree(nums1, nums2, nums3 []int) (ans []int) {
	mask := map[int]int{}
	for i, nums := range [][]int{nums1, nums2, nums3} {
		for _, x := range nums {
			mask[x] |= 1 << i
		}
	}
	for x, m := range mask {
		if m&(m-1) > 0 {
			ans = append(ans, x)
		}
	}
	return
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		nums1 := []int{1, 1, 3, 2}
		nums2 := []int{2, 3}
		nums3 := []int{3}
		ans := []int{3, 2}
		assert(twoOutOfThree(nums1, nums2, nums3), ans)
	}

	{
		nums1 := []int{3, 1}
		nums2 := []int{2, 3}
		nums3 := []int{1, 2}
		ans := []int{1, 2, 3}
		assert(twoOutOfThree(nums1, nums2, nums3), ans)
	}

	{
		nums1 := []int{1, 2, 2}
		nums2 := []int{4, 3, 3}
		nums3 := []int{5}
		ans := []int{}
		assert(twoOutOfThree(nums1, nums2, nums3), ans)
	}
}
