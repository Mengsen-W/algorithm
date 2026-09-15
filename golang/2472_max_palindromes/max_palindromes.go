// Package main ...
package main

import "fmt"

func maxPalindromes(s string, k int) int {
	n := len(s)
	ans, start := 0, 0

	check := func(l, r int) bool {
		for l < r {
			if s[l] != s[r] {
				return false
			}
			l++
			r--
		}
		return true
	}

	for r := k - 1; r < n; r++ {
		l := r - k + 1
		if l >= start && check(l, r) {
			ans++
			start = r + 1
			continue
		}

		l = r - k
		if l >= start && check(l, r) {
			ans++
			start = r + 1
		}
	}

	return ans
}

func main() {
	tests := []struct {
		s   string
		k   int
		ans int
	}{
		{"abaccdbbd", 3, 2},
		{"adbcda", 2, 0},
	}

	for index, test :=range tests {
		if maxPalindromes(test.s, test.k) != test.ans {
			fmt.Println(index)
		}
	}
}
