/*
 * @Date: 2023-03-31
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-31
 * @FilePath: /algorithm/golang/2367_arithmetic_triplets/arithmetic_triplets.go
 */

// Package main ...
package main

func arithmeticTriplets(nums []int, diff int) int {
	ans := 0
	n := len(nums)
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	for i, j, k := 0, 1, 2; i < n-2 && j < n-1 && k < n; i++ {
		j = max(j, i+1)
		for j < n-1 && nums[j]-nums[i] < diff {
			j++
		}
		if j >= n-1 || nums[j]-nums[i] > diff {
			continue
		}
		k = max(k, j+1)
		for k < n && nums[k]-nums[j] < diff {
			k++
		}
		if k < n && nums[k]-nums[j] == diff {
			ans++
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
		nums := []int{0, 1, 4, 6, 7, 10}
		diff := 3
		ans := 2
		assert(arithmeticTriplets(nums, diff) == ans)
	}

	{
		nums := []int{0, 1, 4, 6, 7, 10}
		diff := 3
		ans := 2
		assert(arithmeticTriplets(nums, diff) == ans)
	}

	{
		nums := []int{4, 5, 6, 7, 8, 9}
		diff := 2
		ans := 2
		assert(arithmeticTriplets(nums, diff) == ans)
	}
}
