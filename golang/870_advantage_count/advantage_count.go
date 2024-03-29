/*
 * @Date: 2022-10-08
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-08
 * @FilePath: /algorithm/870_advantage_count/advantage_count.go
 */

package main

import (
	"reflect"
	"sort"
)

func advantageCount(nums1 []int, nums2 []int) []int {
	n := len(nums1)
	idx1 := make([]int, n)
	idx2 := make([]int, n)
	for i := 1; i < n; i++ {
		idx1[i] = i
		idx2[i] = i
	}
	sort.Slice(idx1, func(i, j int) bool { return nums1[idx1[i]] < nums1[idx1[j]] })
	sort.Slice(idx2, func(i, j int) bool { return nums2[idx2[i]] < nums2[idx2[j]] })

	ans := make([]int, n)
	left, right := 0, n-1
	for i := 0; i < n; i++ {
		if nums1[idx1[i]] > nums2[idx2[left]] {
			ans[idx2[left]] = nums1[idx1[i]]
			left++
		} else {
			ans[idx2[right]] = nums1[idx1[i]]
			right--
		}
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
		nums1 := []int{2, 7, 11, 15}
		nums2 := []int{1, 10, 4, 11}
		ans := []int{2, 11, 7, 15}
		assert(advantageCount(nums1, nums2), ans)
	}

	{
		nums1 := []int{12, 24, 8, 32}
		nums2 := []int{13, 25, 32, 11}
		ans := []int{24, 32, 8, 12}
		assert(advantageCount(nums1, nums2), ans)
	}
}
