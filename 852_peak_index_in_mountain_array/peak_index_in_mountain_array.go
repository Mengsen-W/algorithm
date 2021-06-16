/*
 * @Date: 2021-06-15 08:40:28
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-15 08:49:02
 */

package main

import "sort"

func peakIndexInMountainArray(arr []int) int {
	return sort.Search(len(arr)-1, func(i int) bool { return arr[i] > arr[i+1] })
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed!")
		}
	}
	{
		arr := []int{0, 1, 0}
		ans := 1
		assert(peakIndexInMountainArray(arr) == ans)
	}
	{
		arr := []int{0, 2, 1, 0}
		ans := 1
		assert(peakIndexInMountainArray(arr) == ans)
	}
	{
		arr := []int{0, 10, 5, 2, 0}
		ans := 1
		assert(peakIndexInMountainArray(arr) == ans)
	}
	{
		arr := []int{3, 4, 5, 1}
		ans := 2
		assert(peakIndexInMountainArray(arr) == ans)
	}
	{
		arr := []int{24, 69, 100, 99, 79, 78, 67, 36, 26, 19}
		ans := 2
		assert(peakIndexInMountainArray(arr) == ans)
	}
}
