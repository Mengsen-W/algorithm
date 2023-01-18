/*
 * @Date: 2022-02-02 00:53:14
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-02 01:00:46
 */

package main

import "strings"

func reversePrefix(word string, ch byte) string {
	right := strings.IndexByte(word, ch)
	if right < 0 {
		return word
	}
	s := []byte(word)
	for left := 0; left < right; left++ {
		s[left], s[right] = s[right], s[left]
		right--
	}
	return string(s)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(reversePrefix("abcdefd", 'd') == "dcbaefd")
	assert(reversePrefix("xyxzxe", 'z') == "zxyxxe")
	assert(reversePrefix("abcd", 'z') == "abcd")
}
