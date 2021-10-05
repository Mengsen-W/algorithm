/*
 * @Date: 2021-10-04 17:48:31
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-04 17:57:25
 */

package main

import "unicode"

func licenseKeyFormatting(s string, k int) string {
	ans := []byte{}
	for i, cnt := len(s)-1, 0; i >= 0; i-- {
		if s[i] != '-' {
			ans = append(ans, byte(unicode.ToUpper(rune(s[i]))))
			cnt++
			if cnt%k == 0 {
				ans = append(ans, '-')
			}
		}
	}
	if len(ans) > 0 && ans[len(ans)-1] == '-' {
		ans = ans[:len(ans)-1]
	}
	for i, n := 0, len(ans); i < n/2; i++ {
		ans[i], ans[n-1-i] = ans[n-1-i], ans[i]
	}
	return string(ans)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		s := "5F3Z-2e-9-w"
		k := 4
		ans := "5F3Z-2E9W"
		assert(licenseKeyFormatting(s, k) == ans)
	}
	{
		s := "2-5g-3-J"
		k := 2
		ans := "2-5G-3J"
		assert(licenseKeyFormatting(s, k) == ans)
	}
}
