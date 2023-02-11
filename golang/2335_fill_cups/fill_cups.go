/*
 * @Date: 2023-02-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-11
 * @FilePath: /algorithm/golang/2335_fill_cups/fill_cups.go
 */

package main

import "sort"

func fillCups(amount []int) int {
	sort.Ints(amount)
	if amount[2] > amount[1]+amount[0] {
		return amount[2]
	}
	return (amount[0] + amount[1] + amount[2] + 1) / 2
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		amount := []int{1, 4, 2}
		ans := 4
		assert(fillCups(amount) == ans)
	}

	{
		amount := []int{5, 4, 4}
		ans := 7
		assert(fillCups(amount) == ans)
	}

	{
		amount := []int{5, 0, 0}
		ans := 5
		assert(fillCups(amount) == ans)
	}
}
