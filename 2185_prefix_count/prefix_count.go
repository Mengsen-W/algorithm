/*
 * @Date: 2023-01-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-08
 * @FilePath: /algorithm/2185_prefix_count/prefix_count.go
 */

package main

import "strings"

func prefixCount(words []string, pref string) (ans int) {
	for _, word := range words {
		if strings.HasPrefix(word, pref) {
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

	{
		words := []string{"pay", "attention", "practice", "attend"}
		pref := "at"
		ans := 2
		assert(prefixCount(words, pref) == ans)
	}

	{
		words := []string{"leetcode", "win", "loops", "success"}
		pref := "code"
		ans := 0
		assert(prefixCount(words, pref) == ans)
	}
}
