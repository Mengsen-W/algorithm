/*
 * @Date: 2023-05-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-12
 * @FilePath: /algorithm/golang/1016_query_string/query_string.go
 */

// Package main ...
package main

import "strings"

func queryString(s string, n int) bool {
	if n == 1 {
		return strings.Contains(s, "1")
	}
	k := 30
	for (1 << k) >= n {
		k--
	}
	if len(s) < (1<<(k-1))+k-1 || len(s) < n-(1<<k)+k+1 {
		return false
	}

	help := func(s string, k int, mi int, ma int) bool {
		st := map[int]struct{}{}
		t := 0
		for r := 0; r < len(s); r++ {
			t = (t << 1) + int(s[r]-'0')
			if r >= k {
				t -= int(s[r-k]-'0') << k
			}
			if r >= k-1 && t >= mi && t <= ma {
				st[t] = struct{}{}
			}
		}
		return len(st) == ma-mi+1
	}
	return help(s, k, 1<<(k-1), (1<<k)-1) && help(s, k+1, 1<<k, n)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		s := "0110"
		n := 3
		ans := true
		assert(queryString(s, n) == ans)
	}

	{
		s := "0110"
		n := 4
		ans := false
		assert(queryString(s, n) == ans)
	}
}
