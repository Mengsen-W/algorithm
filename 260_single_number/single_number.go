/*
 * @Date: 2021-10-30 01:13:14
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-30 01:25:02
 */

package main

import "reflect"

func singleNumber(nums []int) []int {
	xorSum := 0
	for _, num := range nums {
		xorSum ^= num
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
	return []int{type1, type2}
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("NotPassed")
		}
	}
	assert(singleNumber([]int{1, 2, 1, 3, 2, 5}), []int{3, 5})
	assert(singleNumber([]int{-1, 0}), []int{-1, 0})
	assert(singleNumber([]int{0, 1}), []int{1, 0})
}
