/*
 * @Date: 2021-10-22 00:48:18
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-22 01:06:56
 */

package main

import "reflect"

func majorityElement(nums []int) (ans []int) {
	cnt := map[int]int{}
	for _, v := range nums {
		cnt[v]++
	}
	for v, c := range cnt {
		if c > len(nums)/3 {
			ans = append(ans, v)
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
	assert(majorityElement([]int{3, 2, 3}), []int{3})
	assert(majorityElement([]int{1}), []int{1})
	assert(majorityElement([]int{1, 1, 1, 3, 3, 2, 2, 2}), []int{1, 2})
}
