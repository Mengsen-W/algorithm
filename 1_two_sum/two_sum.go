/*
 * @Date: 2021-08-26 11:19:19
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-26 11:44:39
 */

package main

import (
	"reflect"
)

func twoSum(nums []int, target int) []int {
	hashTable := map[int]int{}
	for i, x := range nums {
		if p, ok := hashTable[target-x]; ok {
			return []int{p, i}
		}
		hashTable[x] = i
	}
	return nil
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	{
		nums := []int{2, 7, 11, 15}
		target := 9
		ans := []int{0, 1}
		assert(twoSum(nums, target), ans)
	}
	{
		nums := []int{3, 2, 4}
		target := 6
		ans := []int{1, 2}
		assert(twoSum(nums, target), ans)
	}
	{
		nums := []int{3, 3}
		target := 6
		ans := []int{0, 1}
		assert(twoSum(nums, target), ans)
	}
}
