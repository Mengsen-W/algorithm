/*
 * @Date: 2022-01-22 09:13:56
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-22 09:19:05
 */

package main

func removePalindromeSub(s string) int {
	for i, n := 0, len(s); i < n/2; i++ {
		if s[i] != s[n-1-i] {
			return 2
		}
	}
	return 1
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(removePalindromeSub("ababa") == 1)
	assert(removePalindromeSub("abb") == 2)
	assert(removePalindromeSub("babb") == 2)
}
