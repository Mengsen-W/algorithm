/*
 * @Date: 2023-03-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-16
 * @FilePath: /algorithm/golang/2488_count_subarrays/count_subarrays.go
 */

// Package main ...
package main

func countSubarrays(nums []int, k int) int {
	sign := func(num int) int {
		if num == 0 {
			return 0
		}
		if num > 0 {
			return 1
		}
		return -1
	}

	kIndex := -1
	for i, num := range nums {
		if num == k {
			kIndex = i
			break
		}
	}
	ans := 0
	counts := map[int]int{}
	counts[0] = 1
	sum := 0
	for i, num := range nums {
		sum += sign(num - k)
		if i < kIndex {
			counts[sum]++
		} else {
			prev0 := counts[sum]
			prev1 := counts[sum-1]
			ans += prev0 + prev1
		}
	}
	return ans
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		nums := []int{3, 2, 1, 4, 5}
		k := 4
		ans := 3
		assert(countSubarrays(nums, k) == ans)
	}

	{
		nums := []int{2, 3, 1}
		k := 3
		ans := 1
		assert(countSubarrays(nums, k) == ans)
	}
}
