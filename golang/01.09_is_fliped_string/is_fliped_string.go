/*
 * @Date: 2022-09-29
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-29
 * @FilePath: /algorithm/01.09_is_fliped_string/is_fliped_string.go
 */

package main

import "strings"

func isFlipedString(s1 string, s2 string) bool {
	return len(s1) == len(s2) && strings.Contains(s1+s1, s2)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(isFlipedString("waterbottle", "erbottlewat"))
	assert(!isFlipedString("aa", "aba"))
}
