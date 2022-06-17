/*
 * @Date: 2022-06-17 09:39:07
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-17 09:47:19
 * @FilePath: /algorithm/1089_duplicate_zeros/duplicate_zeros.go
 */

package main

import "reflect"

func duplicateZeros(arr []int) {
	n := len(arr)
	top := 0
	i := -1
	for top < n {
		i++
		if arr[i] != 0 {
			top++
		} else {
			top += 2
		}
	}
	j := n - 1
	if top == n+1 {
		arr[j] = 0
		j--
		i--
	}
	for j >= 0 {
		arr[j] = arr[i]
		j--
		if arr[i] == 0 {
			arr[j] = arr[i]
			j--
		}
		i--
	}
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	{
		arr := []int{1, 0, 2, 3, 0, 4, 5, 0}
		duplicateZeros(arr)
		assert(arr, []int{1, 0, 0, 2, 3, 0, 0, 4})
	}
	{
		arr := []int{1, 2, 3}
		duplicateZeros(arr)
		assert(arr, []int{1, 2, 3})
	}
}
