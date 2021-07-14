/*
 * @Date: 2021-07-14 08:27:35
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-14 08:45:49
 */

package main

import (
	"sort"
)

func minAbsoluteSumDiff(nums1, nums2 []int) int {
	abs := func(x int) int {
		if x < 0 {
			return -x
		}
		return x
	}
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	rec := append(sort.IntSlice(nil), nums1...)
	rec.Sort()
	sum, maxn, n := 0, 0, len(nums1)
	for i, v := range nums2 {
		diff := abs(nums1[i] - v)
		sum += diff
		j := rec.Search(v)
		if j < n {
			maxn = max(maxn, diff-(rec[j]-v))
		}
		if j > 0 {
			maxn = max(maxn, diff-(v-rec[j-1]))
		}
	}
	return (sum - maxn) % (1e9 + 7)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed!")
		}
	}
	{
		nums1 := []int{1, 7, 5}
		nums2 := []int{2, 3, 5}
		assert(minAbsoluteSumDiff(nums1, nums2) == 3)
	}
	{
		nums1 := []int{2, 4, 6, 8, 10}
		nums2 := []int{2, 4, 6, 8, 10}
		assert(minAbsoluteSumDiff(nums1, nums2) == 0)
	}
	{
		nums1 := []int{1, 10, 4, 4, 2, 7}
		nums2 := []int{9, 3, 5, 1, 7, 4}
		assert(minAbsoluteSumDiff(nums1, nums2) == 20)
	}
}
