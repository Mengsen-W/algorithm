/*
 * @Date: 2021-12-01 00:26:09
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-01 00:36:14
 */

package main

func maxPower(s string) int {
	ans, cnt := 1, 1
	for i := 1; i < len(s); i++ {
		if s[i] == s[i-1] {
			cnt++
			if cnt > ans {
				ans = cnt
			}
		} else {
			cnt = 1
		}
	}
	return ans
}

func main() {
	assert := func(a bool) {
		if !a {
			panic("Not Passed")
		}
	}

	assert(maxPower("leetcode") == 2)
	assert(maxPower("abbcccddddeeeeedcba") == 5)
	assert(maxPower("triplepillooooow") == 5)
	assert(maxPower("hooraaaaaaaaaaay") == 11)
}
