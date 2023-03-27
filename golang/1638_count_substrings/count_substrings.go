/*
 * @Date: 2023-03-27
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-27
 * @FilePath: /algorithm/golang/1638_count_substrings/count_substrings.go
 */

// Package main ...
package main

func countSubstrings(s string, t string) (result int) {
	dp := make([][2]int, len(t)+1)
	for i := 0; i < len(s); i++ {
		for j := len(t) - 1; j >= 0; j-- {
			if s[i] != t[j] {
				dp[j+1][0] = 0
				dp[j+1][1] = dp[j][0] + 1
			} else {
				dp[j+1][0] = dp[j][0] + 1
				dp[j+1][1] = dp[j][1]
			}
			result += dp[j+1][1]
		}
	}
	return result
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		s := "aba"
		t := "baba"
		ans := 6
		assert(countSubstrings(s, t) == ans)
	}

	{
		s := "ab"
		t := "bb"
		ans := 3
		assert(countSubstrings(s, t) == ans)
	}

	{
		s := "a"
		t := "a"
		ans := 0
		assert(countSubstrings(s, t) == ans)
	}

	{
		s := "abe"
		t := "bbc"
		ans := 10
		assert(countSubstrings(s, t) == ans)
	}
}
