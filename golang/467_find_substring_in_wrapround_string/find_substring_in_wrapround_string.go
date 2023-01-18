/*
 * @Date: 2022-05-25 09:55:09
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-25 10:07:19
 * @FilePath: /algorithm/467_find_substring_in_wrapround_string/find_substring_in_wrapround_string.go
 */

package main

func findSubstringInWraproundString(p string) (ans int) {
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}
	dp := [26]int{}
	k := 0
	for i, ch := range p {
		if i > 0 && (byte(ch)-p[i-1]+26)%26 == 1 { // 字符之差为 1 或 -25
			k++
		} else {
			k = 1
		}
		dp[ch-'a'] = max(dp[ch-'a'], k)
	}
	for _, v := range dp {
		ans += v
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(findSubstringInWraproundString("a") == 1)
	assert(findSubstringInWraproundString("cac") == 2)
	assert(findSubstringInWraproundString("zab") == 6)
}
