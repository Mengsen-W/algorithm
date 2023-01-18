/*
 * @Date: 2022-02-19 14:08:10
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-19 14:26:15
 */

package main

import (
	"reflect"
)

func pancakeSort(arr []int) (ans []int) {
	for n := len(arr); n > 1; n-- {
		index := 0
		for i, v := range arr[:n] {
			if v > arr[index] {
				index = i
			}
		}
		if index == n-1 {
			continue
		}
		for i, m := 0, index; i < (m+1)/2; i++ {
			arr[i], arr[m-i] = arr[m-i], arr[i]
		}
		for i := 0; i < n/2; i++ {
			arr[i], arr[n-1-i] = arr[n-1-i], arr[i]
		}
		ans = append(ans, index+1, n)
	}
	return
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	assert(pancakeSort([]int{3, 2, 4, 1}), []int{3, 4, 2, 3, 1, 2})
	assert(pancakeSort([]int{3, 2, 1}), []int{1, 3})

}
