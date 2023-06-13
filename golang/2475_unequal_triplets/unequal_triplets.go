/*
 * @Date: 2023-06-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-13
 * @FilePath: /algorithm/golang/2475_unequal_triplets/unequal_triplets.go
 */

// Package main ...
package main

func unequalTriplets(nums []int) int {
	count := map[int]int{}
	for _, x := range nums {
		count[x]++
	}
	res, n, t := 0, len(nums), 0
	for _, v := range count {
		res, t = res+t*v*(n-t-v), t+v
	}
	return res
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		nums := []int{4, 4, 2, 4, 3}
		ans := 3
		assert(unequalTriplets(nums) == ans)
	}

	{
		nums := []int{1, 1, 1, 1, 1}
		ans := 0
		assert(unequalTriplets(nums) == ans)
	}
}
