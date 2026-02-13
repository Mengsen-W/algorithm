// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func case2Helper(s string, x, y byte) int {
	n := len(s)
	res := 0

	for i := 0; i < n; i++ {
		if s[i] != x && s[i] != y {
			continue
		}

		h := make(map[int]int)
		h[0] = i - 1
		diff := 0
		j := i
		for j < n && (s[j] == x || s[j] == y) {
			if s[j] == x {
				diff++
			} else {
				diff--
			}

			if prev, exists := h[diff]; exists {
				if j-prev > res {
					res = j - prev
				}
			} else {
				h[diff] = j
			}
			j++
		}
		i = j - 1
	}
	return res
}

func longestBalanced(s string) int {
	n := len(s)
	res := 0

	// 情况一，仅包括一种字符
	last := 0
	for i := 0; i < n; i++ {
		if i > 0 && s[i] == s[i-1] {
			last++
		} else {
			last = 1
		}
		if last > res {
			res = last
		}
	}

	// 情况二，包含两种字符
	res = max(res, case2Helper(s, 'a', 'b'))
	res = max(res, case2Helper(s, 'b', 'c'))
	res = max(res, case2Helper(s, 'a', 'c'))

	// 情况三，包含三种字符
	type Key struct {
		x, y int
	}
	h := make(map[Key]int)
	h[Key{n, n}] = -1

	diffAB := 0
	diffBC := 0
	for i := 0; i < n; i++ {
		c := s[i]
		switch c {
		case 'a':
			diffAB--
		case 'b':
			diffAB++
			diffBC++
		case 'c':
			diffBC--
		}

		key := Key{diffAB + n, diffBC + n}
		if prev, exists := h[key]; exists {
			res = max(res, i-prev)
		} else {
			h[key] = i
		}
	}
	return res
}

func main() {
	tests := []struct {
		s   string
		ans int
	}{
		{"abbac", 4},
		{"aabcc", 3},
		{"aba", 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, longestBalanced(test.s), index)
	}
}
