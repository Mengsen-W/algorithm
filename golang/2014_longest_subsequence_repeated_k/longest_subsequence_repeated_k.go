// Package main ...
package main

import "github.com/stretchr/testify/assert"

func longestSubsequenceRepeatedK(s string, k int) string {
	check := func(t string, k int) bool {
		i := 0
		for _, c := range s {
			if byte(c) == t[i] {
				i++
				if i == len(t) {
					k--
					if k == 0 {
						return true
					}
					i = 0
				}
			}
		}
		return false
	}

	cnt := [26]int{}
	for i := 0; i < len(s); i++ {
		cnt[s[i]-'a']++
	}

	cs := []byte{}
	for c := byte('a'); c <= 'z'; c++ {
		if cnt[c-'a'] >= k {
			cs = append(cs, c)
		}
	}

	q := []string{""}
	ans := ""
	for len(q) > 0 {
		cur := q[0]
		q = q[1:]
		for _, c := range cs {
			nxt := cur + string(c)
			if check(nxt, k) {
				ans = nxt
				q = append(q, nxt)
			}
		}
	}
	return ans
}

func main() {
	tests := []struct {
		s        string
		k        int
		expected string
	}{
		{"letsleetcode", 2, "let"},
		{"bb", 2, "b"},
		{"ab", 2, ""},
	}

	for index, test := range tests {
		assert.Equal(nil, longestSubsequenceRepeatedK(test.s, test.k), test.expected,
			index)
	}
}
