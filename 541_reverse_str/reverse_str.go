/*
 * @Date: 2021-08-20 15:23:31
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-20 15:26:33
 */

package main

func reverseStr(s string, k int) string {
	t := []byte(s)
	for i := 0; i < len(s); i += 2 * k {
		sub := t[i:min(i+k, len(s))]
		for j, n := 0, len(sub); j < n/2; j++ {
			sub[j], sub[n-1-j] = sub[n-1-j], sub[j]
		}
	}
	return string(t)
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		s := "abcdefg"
		k := 2
		ans := "bacdfeg"
		assert(reverseStr(s, k) == ans)
	}
	{
		s := "abcd"
		k := 2
		ans := "bacd"
		assert(reverseStr(s, k) == ans)
	}
}
