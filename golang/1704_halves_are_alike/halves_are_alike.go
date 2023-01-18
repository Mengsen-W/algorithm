/*
 * @Date: 2022-11-11
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-11
 * @FilePath: /algorithm/1704_halves_are_alike/halves_are_alike.go
 */

package main

import "strings"

func halvesAreAlike(s string) bool {
	cnt := 0
	for _, c := range s[:len(s)/2] {
		if strings.ContainsRune("aeiouAEIOU", c) {
			cnt++
		}
	}
	for _, c := range s[len(s)/2:] {
		if strings.ContainsRune("aeiouAEIOU", c) {
			cnt--
		}
	}
	return cnt == 0
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(halvesAreAlike("book"))
	assert(!halvesAreAlike("textbook"))
}
