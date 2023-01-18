/*
 * @Date: 2022-08-25
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-25
 * @FilePath: /algorithm/658_find_closest_elements/find_closest_elements.go
 */

package main

import (
	"reflect"
	"sort"
)

func findClosestElements(arr []int, k, x int) []int {
	right := sort.SearchInts(arr, x)
	left := right - 1
	for ; k > 0; k-- {
		if left < 0 {
			right++
		} else if right >= len(arr) || x-arr[left] <= arr[right]-x {
			left--
		} else {
			right++
		}
	}
	return arr[left+1 : right]
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		arr := []int{1, 2, 3, 4, 5}
		k := 4
		x := 3
		ans := []int{1, 2, 3, 4}
		assert(findClosestElements(arr, k, x), ans)
	}

	{
		arr := []int{1, 2, 3, 4, 5}
		k := 4
		x := -1
		ans := []int{1, 2, 3, 4}
		assert(findClosestElements(arr, k, x), ans)
	}
}
