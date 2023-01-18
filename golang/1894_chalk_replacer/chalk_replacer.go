/*
 * @Date: 2021-09-10 09:10:46
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-10 09:21:24
 */

package main

import (
	"sort"
)

func chalkReplacer(chalk []int, k int) int {
	if chalk[0] > k {
		return 0
	}
	n := len(chalk)
	for i := 1; i < n; i++ {
		chalk[i] += chalk[i-1]
		if chalk[i] > k {
			return i
		}
	}
	k %= chalk[n-1]
	return sort.SearchInts(chalk, k+1)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		chalk := []int{5, 1, 5}
		k := 22
		ans := 0
		assert(chalkReplacer(chalk, k) == ans)
	}
	{
		chalk := []int{3, 4, 1, 2}
		k := 55
		ans := 1
		assert(chalkReplacer(chalk, k) == ans)
	}
}
