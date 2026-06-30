// Package main ...
package main

import "fmt"

func numberOfSubstrings(s string) int {
	n := len(s)
	ans := 0
	cnt := make([]int, 3)

	l, r := 0, -1
	for l < n {
		for r < n && (cnt[0] < 1 || cnt[1] < 1 || cnt[2] < 1) {
			r++
			if r == n {
				break
			}
			cnt[s[r]-'a']++
		}
		if r < n {
			ans += n - r
		}
		cnt[s[l]-'a']--
		l++
	}

	return ans
}

func main() {
	tests := []struct {
		s   string
		ans int
	}{
		{"abcabc", 10},
		{"aaacb", 3},
		{"abc", 1},
	}

	for _, test := range tests {
		got := numberOfSubstrings(test.s)
		if got != test.ans {
			fmt.Printf("Test failed: s=%s, expected=%d, got=%d\n", test.s, test.ans, got)
		}
	}
}
