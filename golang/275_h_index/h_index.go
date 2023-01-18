/*
 * @Date: 2021-07-12 08:06:52
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-12 08:13:30
 */

package main

import "sort"

func hIndex(citations []int) int {
	n := len(citations)
	return n - sort.Search(n, func(x int) bool { return citations[x] >= n-x })
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed!")
		}
	}
	citations := []int{0, 1, 3, 5, 6}
	assert(hIndex(citations) == 3)
}
