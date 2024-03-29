/*
 * @Date: 2022-06-10 09:55:03
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-10 10:15:00
 * @FilePath: /algorithm/730_count_palindromic_subsequences/count_palindromic_subsequences.go
 */

package main

func countPalindromicSubsequences(s string) (ans int) {
	const mod int = 1e9 + 7
	n := len(s)
	dp := make([][]int, n)
	for i := range dp {
		dp[i] = make([]int, n)
		dp[i][i] = 1
	}

	for sz := 2; sz <= n; sz++ {
		for i, j := 0, sz-1; j < n; i++ {
			if s[i] == s[j] {
				low, high := i+1, j-1
				for low <= high && s[low] != s[i] {
					low++
				}
				for high >= low && s[high] != s[j] {
					high--
				}
				if low > high {
					dp[i][j] = (2 + dp[i+1][j-1]*2) % mod
				} else if low == high {
					dp[i][j] = (1 + dp[i+1][j-1]*2) % mod
				} else {
					dp[i][j] = (dp[i+1][j-1]*2 - dp[low+1][high-1] + mod) % mod
				}
			} else {
				dp[i][j] = (dp[i+1][j] + dp[i][j-1] - dp[i+1][j-1] + mod) % mod
			}
			j++
		}
	}

	return dp[0][n-1]
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(countPalindromicSubsequences("bccb") == 6)
	assert(countPalindromicSubsequences("abcdabcdabcdabcdabcdabcdabcdabcddcbadcbadcbadcbadcbadcbadcbadcba") ==
		104860361)
}
