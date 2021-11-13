/*
 * @Date: 2021-11-13 01:15:51
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-13 01:27:33
 */

package main

import "unicode"

func detectCapitalUse(word string) bool {
	// 若第 1 个字母为小写，则需额外判断第 2 个字母是否为小写
	if len(word) >= 2 && unicode.IsLower(rune(word[0])) && unicode.IsUpper(rune(word[1])) {
		return false
	}

	// 无论第 1 个字母是否大写，其他字母必须与第 2 个字母的大小写相同
	for i := 2; i < len(word); i++ {
		if unicode.IsLower(rune(word[i])) != unicode.IsLower(rune(word[1])) {
			return false
		}
	}
	return true
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("assertion failed")
		}
	}
	assert(detectCapitalUse("USA"))
	assert(!detectCapitalUse("FlaG"))
}
