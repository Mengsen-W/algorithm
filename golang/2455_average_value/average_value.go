/*
 * @Date: 2023-05-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-29
 * @FilePath: /algorithm/golang/2455_average_value/average_value.go
 */

// Package main ...
package main

func averageValue(nums []int) int {
	total := 0
	k := 0
	for _, a := range nums {
		if a%6 == 0 {
			total += a
			k++
		}
	}
	if k > 0 {
		return total / k
	}
	return 0
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		nums := []int{1, 3, 6, 10, 12, 15}
		ans := 9
		assert(averageValue(nums) == ans)
	}

	{
		nums := []int{1, 2, 4, 7, 10}
		ans := 0
		assert(averageValue(nums) == ans)
	}
}
