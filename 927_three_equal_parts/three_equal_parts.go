/*
 * @Date: 2022-10-06
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-06
 * @FilePath: /algorithm/927_three_equal_parts/three_equal_parts.go
 */

package main

import "reflect"

func threeEqualParts(arr []int) []int {
	sum := 0
	for _, v := range arr {
		sum += v
	}
	if sum%3 != 0 {
		return []int{-1, -1}
	}
	if sum == 0 {
		return []int{0, 2}
	}

	partial := sum / 3
	first, second, third, cur := 0, 0, 0, 0
	for i, x := range arr {
		if x == 1 {
			if cur == 0 {
				first = i
			} else if cur == partial {
				second = i
			} else if cur == 2*partial {
				third = i
			}
			cur++
		}
	}

	n := len(arr)
	length := n - third
	if first+length <= second && second+length <= third {
		i := 0
		for third+i < n {
			if arr[first+i] != arr[second+i] || arr[first+i] != arr[third+i] {
				return []int{-1, -1}
			}
			i++
		}
		return []int{first + length - 1, second + length}
	}
	return []int{-1, -1}
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		arr := []int{1, 0, 1, 0, 1}
		ans := []int{0, 3}
		assert(threeEqualParts(arr), ans)
	}

	{
		arr := []int{1, 1, 0, 1, 1}
		ans := []int{-1, -1}
		assert(threeEqualParts(arr), ans)
	}

	{
		arr := []int{1, 1, 0, 0, 1}
		ans := []int{0, 2}
		assert(threeEqualParts(arr), ans)
	}
}
