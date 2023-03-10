/*
 * @Date: 2023-03-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-10
 * @FilePath: /algorithm/golang/1590_min_subarray/min_subarray.go
 */

package main

func minSubarray(nums []int, p int) int {
	sum := 0
	mp := map[int]int{0: -1}
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}

	for _, v := range nums {
		sum += v
	}
	rem := sum % p
	if rem == 0 {
		return 0
	}
	minCount := len(nums)
	sum = 0
	for i := 0; i < len(nums); i++ {
		sum += nums[i]
		tempRem := sum % p
		k := (tempRem - rem + p) % p
		if _, ok := mp[k]; ok {
			minCount = min(minCount, i-mp[k])
		}
		mp[tempRem] = i
	}

	if minCount >= len(nums) {
		return -1
	}

	return minCount
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		nums := []int{3, 1, 4, 2}
		p := 6
		ans := 1
		assert(minSubarray(nums, p) == ans)
	}

	{
		nums := []int{6, 3, 5, 2}
		p := 9
		ans := 2
		assert(minSubarray(nums, p) == ans)
	}

	{
		nums := []int{1, 2, 3}
		p := 3
		ans := 0
		assert(minSubarray(nums, p) == ans)
	}

	{
		nums := []int{1, 2, 3}
		p := 7
		ans := -1
		assert(minSubarray(nums, p) == ans)
	}

	{
		nums := []int{1000000000, 1000000000, 1000000000}
		p := 3
		ans := 0
		assert(minSubarray(nums, p) == ans)
	}
}
