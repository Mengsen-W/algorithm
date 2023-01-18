/*
 * @Date: 2022-12-16
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-16
 * @FilePath: /algorithm/1785_min_elements/min_elements.go
 */

package main

func minElements(nums []int, limit, goal int) int {
	sum := 0

	for _, x := range nums {
		sum += x
	}

	abs := func(x int) int {
		if x < 0 {
			return -x
		}
		return x
	}

	diff := abs(sum - goal)
	return (diff + limit - 1) / limit
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		nums := []int{1, -1, 1}
		limit := 3
		goal := -4
		ans := 2
		assert(minElements(nums, limit, goal) == ans)
	}

	{
		nums := []int{1, -10, 9, 1}
		limit := 100
		goal := 0
		ans := 1
		assert(minElements(nums, limit, goal) == ans)
	}
}
