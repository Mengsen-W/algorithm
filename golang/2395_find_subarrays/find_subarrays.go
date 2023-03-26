/*
 * @Date: 2023-03-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-26
 * @FilePath: /algorithm/golang/2395_find_subarrays/find_subarrays.go
 */

// Package main ...
package main

func findSubarrays(nums []int) bool {
	seen := map[int]bool{}
	for i := 1; i < len(nums); i++ {
		sum := nums[i-1] + nums[i]
		if seen[sum] {
			return true
		}
		seen[sum] = true
	}
	return false
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		nums := []int{4, 2, 4}
		ans := true
		assert(findSubarrays(nums) == ans)
	}

	{
		nums := []int{1, 2, 3, 4, 5}
		ans := false
		assert(findSubarrays(nums) == ans)
	}

	{
		nums := []int{0, 0, 0}
		ans := true
		assert(findSubarrays(nums) == ans)
	}
}
