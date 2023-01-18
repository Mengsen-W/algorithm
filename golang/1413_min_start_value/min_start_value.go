/*
 * @Date: 2022-08-09
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-09
 * @FilePath: /algorithm/1413_min_start_value/min_start_value.go
 */

package main

import "sort"

func minStartValue(nums []int) int {
	m := nums[0]
	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}
	for _, num := range nums[1:] {
		m = min(m, num)
	}
	return 1 + sort.Search(-m*len(nums), func(startValue int) bool {
		startValue++
		for _, num := range nums {
			startValue += num
			if startValue <= 0 {
				return false
			}
		}
		return true
	})
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(minStartValue([]int{-3, 2, -3, 4, 2}) == 5)
	assert(minStartValue([]int{1, 2}) == 1)
	assert(minStartValue([]int{1, -2, -3}) == 5)
}
