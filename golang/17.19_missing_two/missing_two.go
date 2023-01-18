/*
 * @Date: 2022-09-26
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-26
 * @FilePath: /algorithm/17.19_missing_two/missing_two.go
 */

package main

import "reflect"

func missingTwo(nums []int) []int {
	xorSum := 0
	n := len(nums) + 2
	for _, num := range nums {
		xorSum ^= num
	}
	for i := 1; i <= n; i++ {
		xorSum ^= i
	}
	lsb := xorSum & -xorSum
	type1, type2 := 0, 0
	for _, num := range nums {
		if num&lsb > 0 {
			type1 ^= num
		} else {
			type2 ^= num
		}
	}
	for i := 1; i <= n; i++ {
		if i&lsb > 0 {
			type1 ^= i
		} else {
			type2 ^= i
		}
	}
	return []int{type1, type2}
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	{
		nums := []int{1}
		ans := []int{3, 2}
		assert(missingTwo(nums), ans)
	}

	{
		nums := []int{2, 3}
		ans := []int{1, 4}
		assert(missingTwo(nums), ans)
	}
}
