/*
 * @Date: 2022-10-10
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-10
 * @FilePath: /algorithm/801_min_swap/min_swap.go
 */

package main

func minSwap(nums1, nums2 []int) int {
	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}
	n := len(nums1)
	a, b := 0, 1
	for i := 1; i < n; i++ {
		at, bt := a, b
		a, b = n, n
		if nums1[i] > nums1[i-1] && nums2[i] > nums2[i-1] {
			a = min(a, at)
			b = min(b, bt+1)
		}
		if nums1[i] > nums2[i-1] && nums2[i] > nums1[i-1] {
			a = min(a, bt)
			b = min(b, at+1)
		}
	}
	return min(a, b)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		nums1 := []int{1, 3, 5, 4}
		nums2 := []int{1, 2, 3, 7}
		ans := 1
		assert(minSwap(nums1, nums2) == ans)
	}

	{
		nums1 := []int{0, 3, 5, 8, 9}
		nums2 := []int{2, 1, 4, 6, 9}
		ans := 1
		assert(minSwap(nums1, nums2) == ans)
	}
}
