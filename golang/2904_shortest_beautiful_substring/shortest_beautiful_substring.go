// Package main ...
package main

import "strings"

func shortestBeautifulSubstring(s string, k int) string {
	if strings.Count(s, "1") < k {
		return ""
	}
	ans := s
	cnt, left := 0, 0
	for right := 0; right < len(s); right++ {
		cnt += int(s[right] - '0')
		for cnt > k || s[left] == '0' {
			cnt -= int(s[left] - '0')
			left++
		}
		if cnt == k {
			t := s[left : right+1]
			if len(t) < len(ans) || len(t) == len(ans) && t < ans {
				ans = t
			}
		}
	}
	return ans
}

func main() {
	tests := []struct {
		s   string
		k   int
		ans string
	}{
		{"100011001", 3, "11001"},
		{"1011", 2, "11"},
		{"000", 1, ""},
	}

	for _, test := range tests {
		if got := shortestBeautifulSubstring(test.s, test.k); got != test.ans {
			panic("got " + got + " want " + test.ans)
		}
	}
}
