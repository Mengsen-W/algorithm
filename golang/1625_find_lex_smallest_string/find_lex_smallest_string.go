/*
 * @Date: 2023-03-19
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-19
 * @FilePath: /algorithm/golang/1625_find_lex_smallest_string/find_lex_smallest_string.go
 */

// Package main ...
package main

func findLexSmallestString(s string, a int, b int) string {
	q := []string{s}
	vis := map[string]bool{s: true}
	ans := s
	n := len(s)
	for len(q) > 0 {
		s = q[0]
		q = q[1:]
		if ans > s {
			ans = s
		}
		t1 := []byte(s)
		for i := 1; i < n; i += 2 {
			t1[i] = byte((int(t1[i]-'0')+a)%10 + '0')
		}
		t2 := s[n-b:] + s[:n-b]
		for _, t := range []string{string(t1), t2} {
			if !vis[t] {
				vis[t] = true
				q = append(q, t)
			}
		}
	}
	return ans
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		s := "5525"
		a := 9
		b := 2
		ans := "2050"
		assert(findLexSmallestString(s, a, b) == ans)
	}

	{
		s := "74"
		a := 5
		b := 1
		ans := "24"
		assert(findLexSmallestString(s, a, b) == ans)
	}

	{
		s := "0011"
		a := 4
		b := 2
		ans := "0011"
		assert(findLexSmallestString(s, a, b) == ans)
	}

	{
		s := "43987654"
		a := 7
		b := 3
		ans := "00553311"
		assert(findLexSmallestString(s, a, b) == ans)
	}
}
