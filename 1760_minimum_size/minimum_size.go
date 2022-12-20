/*
 * @Date: 2022-12-20
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-20
 * @FilePath: /algorithm/1760_minimum_size/minimum_size.go
 */

package main

import "sort"

func minimumSize(nums []int, maxOperations int) int {
	max := 0
	for _, x := range nums {
		if x > max {
			max = x
		}
	}
	return sort.Search(max, func(y int) bool {
		if y == 0 {
			return false
		}
		ops := 0
		for _, x := range nums {
			ops += (x - 1) / y
		}
		return ops <= maxOperations
	})
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		nums := []int{9}
		maxOperations := 2
		ans := 3
		assert(minimumSize(nums, maxOperations) == ans)
	}

	{
		nums := []int{2, 4, 8, 2}
		maxOperations := 4
		ans := 2
		assert(minimumSize(nums, maxOperations) == ans)
	}

	{
		nums := []int{7, 17}
		maxOperations := 2
		ans := 7
		assert(minimumSize(nums, maxOperations) == ans)
	}
}
