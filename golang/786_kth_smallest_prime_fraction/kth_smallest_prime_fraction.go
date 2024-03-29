/*
 * @Date: 2021-11-29 03:24:39
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-29 03:41:44
 */

package main

import "reflect"

func kthSmallestPrimeFraction(arr []int, k int) []int {
	n := len(arr)
	left, right := 0., 1.
	for {
		mid := (left + right) / 2
		i, count := -1, 0
		x, y := 0, 1

		for j := 1; j < n; j++ {
			for float64(arr[i+1])/float64(arr[j]) < mid {
				i++
				if arr[i]*y > arr[j]*x {
					x, y = arr[i], arr[j]
				}
			}
			count += i + 1
		}

		if count == k {
			return []int{x, y}
		}
		if count < k {
			left = mid
		} else {
			right = mid
		}
	}
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	assert(kthSmallestPrimeFraction([]int{1, 2, 3, 5}, 3), []int{2, 5})
	assert(kthSmallestPrimeFraction([]int{1, 7}, 1), []int{1, 7})
}
