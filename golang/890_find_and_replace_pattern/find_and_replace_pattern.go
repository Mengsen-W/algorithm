/*
 * @Date: 2022-06-12 11:04:04
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-12 11:17:20
 * @FilePath: /algorithm/890_find_and_replace_pattern/find_and_replace_pattern.go
 */

package main

import "reflect"

func match(word, pattern string) bool {
	mp := map[rune]byte{}
	for i, x := range word {
		y := pattern[i]
		if mp[x] == 0 {
			mp[x] = y
		} else if mp[x] != y { // word 中的同一字母必须映射到 pattern 中的同一字母上
			return false
		}
	}
	return true
}

func findAndReplacePattern(words []string, pattern string) (ans []string) {
	for _, word := range words {
		if match(word, pattern) && match(pattern, word) {
			ans = append(ans, word)
		}
	}
	return
}

func main() {
	assert := func(a, b []string) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	words := []string{"abc", "deq", "mee", "aqq", "dkd", "ccc"}
	pattern := "abb"
	ans := []string{"mee", "aqq"}
	assert(findAndReplacePattern(words, pattern), ans)
}
