/*
 * @Date: 2023-04-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-24
 * @FilePath: /algorithm/golang/1063_last_substring/last_substring.go
 */

// Package main ...
package main

func lastSubstring(s string) string {
	i, j, n := 0, 1, len(s)
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	for j < n {
		k := 0
		for j+k < n && s[i+k] == s[j+k] {
			k++
		}
		if j+k < n && s[i+k] < s[j+k] {
			i, j = j, max(j+1, i+k+1)
		} else {
			j = j + k + 1
		}
	}
	return s[i:]
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		s := "abab"
		ans := "bab"
		assert(lastSubstring(s) == ans)
	}

	{
		s := "leetcode"
		ans := "tcode"
		assert(lastSubstring(s) == ans)
	}
}
