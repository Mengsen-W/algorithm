/*
 * @Date: 2022-08-11
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-11
 * @FilePath: /algorithm/1417_reformat/reformat.go
 */

package main

import "unicode"

func reformat(s string) string {
	sumDigit := 0
	for _, c := range s {
		if unicode.IsDigit(c) {
			sumDigit++
		}
	}
	sumAlpha := len(s) - sumDigit
	if func(x int) int {
		if x < 0 {
			return -x
		}
		return x
	}(sumDigit-sumAlpha) > 1 {
		return ""
	}
	flag := sumDigit > sumAlpha
	t := []byte(s)
	for i, j := 0, 1; i < len(t); i += 2 {
		if unicode.IsDigit(rune(t[i])) != flag {
			for unicode.IsDigit(rune(t[j])) != flag {
				j += 2
			}
			t[i], t[j] = t[j], t[i]
		}
	}
	return string(t)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(reformat("a0b1c2") == "a0b1c2")
	assert(reformat("leetcode") == "")
	assert(reformat("1229857369") == "")
	assert(reformat("covid2019") == "c2o0v1i9d")
	assert(reformat("ab123") == "1a2b3")
}
