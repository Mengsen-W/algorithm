/*
 * @Date: 2021-08-19 09:44:44
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-19 09:56:08
 */

package main

import (
	"strings"
)

func reverseVowels(s string) string {
	t := []byte(s)
	n := len(t)
	i, j := 0, n-1
	for i < j {
		for i < n && !strings.Contains("aeiouAEIOU", string(t[i])) {
			i++
		}
		for j > 0 && !strings.Contains("aeiouAEIOU", string(t[j])) {
			j--
		}
		if i < j {
			t[i], t[j] = t[j], t[i]
			i++
			j--
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
	{
		s := "hello"
		ans := "holle"
		assert(reverseVowels(s) == ans)
	}
	{
		s := "leetcode"
		ans := "leotcede"
		assert(reverseVowels(s) == ans)
	}
}
