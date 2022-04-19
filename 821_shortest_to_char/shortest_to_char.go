/*
 * @Date: 2022-04-19 07:19:16
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-19 07:25:44
 * @FilePath: /algorithm/821_shortest_to_char/shortest_to_char.go
 */

package main

import "reflect"

func shortestToChar(s string, c byte) []int {
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}

	n := len(s)
	ans := make([]int, n)

	idx := -n
	for i, ch := range s {
		if byte(ch) == c {
			idx = i
		}
		ans[i] = i - idx
	}

	idx = n * 2
	for i := n - 1; i >= 0; i-- {
		if s[i] == c {
			idx = i
		}
		ans[i] = min(ans[i], idx-i)
	}
	return ans
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	assert(shortestToChar("loveleetcode", 'e'), []int{3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0})
	assert(shortestToChar("aaab", 'b'), []int{3, 2, 1, 0})
}
