/*
 * @Date: 2023-04-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-01
 * @FilePath: /algorithm/golang/831_maskPII/maskPII.go
 */

// Package main ...
package main

import (
	"strings"
	"unicode"
)

func maskPII(s string) string {
	at := strings.Index(s, "@")
	if at > 0 {
		s = strings.ToLower(s)
		return strings.ToLower(string(s[0])) + "*****" + s[at-1:]
	}
	var sb strings.Builder
	for i := 0; i < len(s); i++ {
		c := s[i]
		if unicode.IsDigit(rune(c)) {
			sb.WriteByte(c)
		}
	}
	s = sb.String()
	country := []string{"", "+*-", "+**-", "+***-"}
	return country[len(s)-10] + "***-***-" + s[len(s)-4:]
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		s := "LeetCode@LeetCode.com"
		ans := "l*****e@leetcode.com"
		assert(maskPII(s) == ans)
	}

	{
		s := "AB@qq.com"
		ans := "a*****b@qq.com"
		assert(maskPII(s) == ans)
	}

	{
		s := "1(234)567-890"
		ans := "***-***-7890"
		assert(maskPII(s) == ans)
	}

	{
		s := "86-(10)12345678"
		ans := "+**-***-***-5678"
		assert(maskPII(s) == ans)
	}
}
