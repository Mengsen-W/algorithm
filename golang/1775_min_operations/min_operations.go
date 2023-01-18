/*
 * @Date: 2022-12-07
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-07
 * @FilePath: /algorithm/1775_min_operations/min_operations.go
 */

package main

func minOperations(nums1 []int, nums2 []int) (ans int) {

	n, m := len(nums1), len(nums2)
	if 6*n < m || 6*m < n {
		return -1
	}
	var cnt1, cnt2 [7]int
	diff := 0
	for _, i := range nums1 {
		cnt1[i]++
		diff += i
	}
	for _, i := range nums2 {
		cnt2[i]++
		diff -= i
	}
	if diff == 0 {
		return 0
	}

	help := func(h1 [7]int, h2 [7]int, diff int) (res int) {
		min := func(a, b int) int {
			if a > b {
				return b
			}
			return a
		}
		h := [7]int{}
		for i := 1; i < 7; i++ {
			h[6-i] += h1[i]
			h[i-1] += h2[i]
		}
		for i := 5; i > 0 && diff > 0; i-- {
			t := min((diff+i-1)/i, h[i])
			res += t
			diff -= t * i
		}
		return res
	}

	if diff > 0 {
		return help(cnt2, cnt1, diff)
	}
	return help(cnt1, cnt2, -diff)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		nums1 := []int{1, 2, 3, 4, 5, 6}
		nums2 := []int{1, 1, 2, 2, 2, 2}
		ans := 3
		assert(minOperations(nums1, nums2) == ans)
	}

	{
		nums1 := []int{1, 1, 1, 1, 1, 1, 1}
		nums2 := []int{6}
		ans := -1
		assert(minOperations(nums1, nums2) == ans)
	}

	{
		nums1 := []int{6, 6}
		nums2 := []int{1}
		ans := 3
		assert(minOperations(nums1, nums2) == ans)
	}
}
