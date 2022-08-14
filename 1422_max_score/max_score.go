/*
 * @Date: 2022-08-14
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-14
 * @FilePath: /algorithm/1422_max_score/max_score.go
 */

package main

import "strings"

func maxScore(s string) int {
	score := int('1'-s[0]) + strings.Count(s[1:], "1")
	ans := score
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}
	for _, c := range s[1 : len(s)-1] {
		if c == '0' {
			score++
		} else {
			score--
		}
		ans = max(ans, score)
	}
	return ans
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(maxScore("011101") == 5)
	assert(maxScore("00111") == 5)
	assert(maxScore("1111") == 3)
}
