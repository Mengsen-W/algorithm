/*
 * @Date: 2021-07-26 10:38:52
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-26 12:48:51
 */

package main

import "sort"

func minOperations(target, arr []int) int {
	n := len(target)
	pos := make(map[int]int, n)
	for i, val := range target {
		pos[val] = i
	}
	d := []int{}
	for _, val := range arr {
		if idx, has := pos[val]; has {
			if p := sort.SearchInts(d, idx); p < len(d) {
				d[p] = idx
			} else {
				d = append(d, idx)
			}
		}
	}
	return n - len(d)
}

func main() {

	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		target := []int{5, 1, 3}
		arr := []int{9, 4, 2, 3, 4}
		assert(minOperations(target, arr) == 2)
	}

	{

		target := []int{5, 1, 3}
		arr := []int{9, 4, 2, 3, 4}
		assert(minOperations(target, arr) == 2)
	}
}
