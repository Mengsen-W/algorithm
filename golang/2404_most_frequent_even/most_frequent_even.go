/*
 * @Date: 2023-04-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-14
 * @FilePath: /algorithm/golang/2404_most_frequent_even/most_frequent_even.go
 */

// Package main ...
package main

func mostFrequentEven(nums []int) int {
	count := map[int]int{}
	for _, x := range nums {
		if x%2 == 0 {
			count[x]++
		}
	}
	res, ct := -1, 0
	for k, v := range count {
		if res == -1 || ct < v || ct == v && k < res {
			res = k
			ct = v
		}
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
		nums := []int{0, 1, 2, 2, 4, 4, 1}
		ans := 2
		assert(mostFrequentEven(nums) == ans)
	}

	{
		nums := []int{4, 4, 4, 9, 2, 4}
		ans := 4
		assert(mostFrequentEven(nums) == ans)
	}

	{
		nums := []int{29, 47, 21, 41, 13, 37, 25, 7}
		ans := -1
		assert(mostFrequentEven(nums) == ans)
	}
}
