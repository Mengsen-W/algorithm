/*
 * @Date: 2022-11-25
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-25
 * @FilePath: /algorithm/809_expressive_words/expressive_words.go
 */

package main

func expressiveWords(s string, words []string) (ans int) {
	expand := func(s, t string) bool {
		n, m := len(s), len(t)
		i, j := 0, 0
		for i < n && j < m {
			if s[i] != t[j] {
				return false
			}
			ch := s[i]
			cntI := 0
			for i < n && s[i] == ch {
				cntI++
				i++
			}
			cntJ := 0
			for j < m && t[j] == ch {
				cntJ++
				j++
			}
			if cntI < cntJ || cntI > cntJ && cntI < 3 {
				return false
			}
		}
		return i == n && j == m
	}

	for _, word := range words {
		if expand(s, word) {
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
	s := "heeellooo"
	words := []string{"hello", "hi", "helo"}
	ans := 1
	assert(expressiveWords(s, words) == ans)
}
