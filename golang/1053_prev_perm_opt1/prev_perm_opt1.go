/*
 * @Date: 2023-04-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-03
 * @FilePath: /algorithm/golang/1053_prev_perm_opt1/prev_perm_opt1.go
 */

// Package main ...
package main

import "reflect"

func prevPermOpt1(arr []int) []int {
	n := len(arr)
	for i := n - 2; i >= 0; i-- {
		if arr[i] > arr[i+1] {
			j := n - 1
			for arr[j] >= arr[i] || arr[j] == arr[j-1] {
				j--
			}
			arr[i], arr[j] = arr[j], arr[i]
			break
		}
	}
	return arr
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	{
		arr := []int{3, 2, 1}
		ans := []int{3, 1, 2}
		assert(prevPermOpt1(arr), ans)
	}

	{
		arr := []int{1, 1, 5}
		ans := []int{1, 1, 5}
		assert(prevPermOpt1(arr), ans)
	}

	{
		arr := []int{1, 9, 4, 6, 7}
		ans := []int{1, 7, 4, 6, 9}
		assert(prevPermOpt1(arr), ans)
	}
}
