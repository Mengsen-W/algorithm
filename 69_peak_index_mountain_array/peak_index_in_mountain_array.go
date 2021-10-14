/*
 * @Date: 2021-10-14 08:47:24
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-14 08:58:56
 */

package main

import "sort"

func peakIndexInMountainArray(arr []int) int {
	return sort.Search(len(arr)-1, func(i int) bool { return arr[i] > arr[i+1] })
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(peakIndexInMountainArray([]int{0, 1, 0}) == 1)
	assert(peakIndexInMountainArray([]int{1, 3, 5, 4, 2}) == 2)
	assert(peakIndexInMountainArray([]int{0, 10, 5, 2}) == 1)
	assert(peakIndexInMountainArray([]int{3, 4, 5, 1}) == 2)
	assert(peakIndexInMountainArray(
		[]int{24, 69, 100, 99, 79, 78, 67, 36, 26, 19}) == 2)
}
