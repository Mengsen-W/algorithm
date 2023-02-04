/*
 * @Date: 2023-02-04
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-04
 * @FilePath: /algorithm/golang/1798_get_maximum_consecutive/get_maximum_consecutive.go
 */

package main

import "sort"

func getMaximumConsecutive(coins []int) int {
	sort.Ints(coins)

	r := 1
	for _, coin := range coins {
		if coin > r {
			break
		}
		r += coin
	}

	return r
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		coins := []int{1, 3}
		ans := 2
		assert(getMaximumConsecutive(coins) == ans)
	}

	{
		coins := []int{1, 1, 1, 4}
		ans := 8
		assert(getMaximumConsecutive(coins) == ans)
	}

	{
		coins := []int{1, 4, 10, 3, 1}
		ans := 20
		assert(getMaximumConsecutive(coins) == ans)
	}
}
