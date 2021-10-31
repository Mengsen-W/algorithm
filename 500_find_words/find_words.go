/*
 * @Date: 2021-10-31 01:51:23
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-31 01:53:26
 */

package main

import (
	"reflect"
	"unicode"
)

func findWords(words []string) (ans []string) {
	const rowIdx = "12210111011122000010020202"
next:
	for _, word := range words {
		idx := rowIdx[unicode.ToLower(rune(word[0]))-'a']
		for _, ch := range word[1:] {
			if rowIdx[unicode.ToLower(ch)-'a'] != idx {
				continue next
			}
		}
		ans = append(ans, word)
	}
	return
}

func main() {
	assert := func(a, b []string) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	assert(findWords([]string{"Hello", "Alaska", "Dad", "Peace"}), []string{"Alaska", "Dad"})
	assert(findWords([]string{"omk"}), nil)
	assert(findWords([]string{"adsdf", "sfd"}), []string{"adsdf", "sfd"})
}
