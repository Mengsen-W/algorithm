/*
 * @Date: 2022-02-01 04:02:39
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-01 04:15:34
 */

package main

import (
	"math/bits"
	"unicode"
)

func longestNiceSubstring(s string) (ans string) {
	mask := uint(0)
	for _, ch := range s {
		mask |= 1 << (unicode.ToLower(ch) - 'a')
	}
	maxTypeNum := bits.OnesCount(mask)

	for typeNum := 1; typeNum <= maxTypeNum; typeNum++ {
		var lowerCnt, upperCnt [26]int
		var total, cnt, l int
		for r, ch := range s {
			idx := unicode.ToLower(ch) - 'a'
			if unicode.IsLower(ch) {
				lowerCnt[idx]++
				if lowerCnt[idx] == 1 && upperCnt[idx] > 0 {
					cnt++
				}
			} else {
				upperCnt[idx]++
				if upperCnt[idx] == 1 && lowerCnt[idx] > 0 {
					cnt++
				}
			}
			if lowerCnt[idx]+upperCnt[idx] == 1 {
				total++
			}

			for total > typeNum {
				idx := unicode.ToLower(rune(s[l])) - 'a'
				if lowerCnt[idx]+upperCnt[idx] == 1 {
					total--
				}
				if unicode.IsLower(rune(s[l])) {
					lowerCnt[idx]--
					if lowerCnt[idx] == 0 && upperCnt[idx] > 0 {
						cnt--
					}
				} else {
					upperCnt[idx]--
					if upperCnt[idx] == 0 && lowerCnt[idx] > 0 {
						cnt--
					}
				}
				l++
			}

			if cnt == typeNum && r-l+1 > len(ans) {
				ans = s[l : r+1]
			}
		}
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(longestNiceSubstring("YazaAay") == "aAa")
	assert(longestNiceSubstring("Bb") == "Bb")
	assert(longestNiceSubstring("c") == "")
	assert(longestNiceSubstring("dDzeE") == "dD")
}
