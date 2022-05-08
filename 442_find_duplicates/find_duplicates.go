/*
 * @Date: 2022-05-08 08:04:34
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-08 08:12:56
 * @FilePath: /algorithm/442_find_duplicates/find_duplicates.go
 */

package main

import "reflect"

func findDuplicates(nums []int) (ans []int) {
	for _, x := range nums {
		if x < 0 {
			x = -x
		}
		if nums[x-1] > 0 {
			nums[x-1] = -nums[x-1]
		} else {
			ans = append(ans, x)
		}
	}
	return
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	assert(findDuplicates([]int{4, 3, 2, 7, 8, 2, 3, 1}), []int{2, 3})
	assert(findDuplicates([]int{1, 1, 2}), []int{1})
	assert(findDuplicates([]int{1}), nil)
}
