/*
 * @Date: 2021-06-13 09:35:06
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-13 09:52:07
 */

package main

import "sort"

func isBadVersion(n int) bool {
	return n == 4
}

func firstBadVersion(n int) int {
	return sort.Search(n, func(version int) bool { return isBadVersion(version) })
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed!")
		}
	}
	assert(firstBadVersion(5) == 4)
}
