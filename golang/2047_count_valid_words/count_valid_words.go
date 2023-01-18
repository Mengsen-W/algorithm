/*
 * @Date: 2022-01-27 02:37:41
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-27 03:01:26
 */

package main

import (
	"strings"
	"unicode"
)

func valid(s string) bool {
	hasHyphens := false
	for i, ch := range s {
		if unicode.IsDigit(ch) || strings.ContainsRune("!.,", ch) && i < len(s)-1 {
			return false
		}
		if ch == '-' {
			if hasHyphens || i == 0 || i == len(s)-1 || !unicode.IsLower(rune(s[i-1])) || !unicode.IsLower(rune(s[i+1])) {
				return false
			}
			hasHyphens = true
		}
	}
	return true
}

func countValidWords(sentence string) (ans int) {
	for _, s := range strings.Fields(sentence) { // 按照空格分割
		if valid(s) {
			ans++
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

	assert(countValidWords("cat and  dog") == 3)
	assert(countValidWords("!this  1-s b8d!") == 0)
	assert(countValidWords(
		"alice and  bob are playing stone-game10") == 5)
	assert(countValidWords(
		"he bought 2 pencils, 3 erasers, and 1  pencil-sharpener.") == 6)
}
