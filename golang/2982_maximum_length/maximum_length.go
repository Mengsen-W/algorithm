// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximumLength(s string) int {
	n := len(s)
	cnt := make(map[byte][]int)
	for i, j := 0, 0; i < n; i = j {
		for j < n && s[j] == s[i] {
			j++
		}
		cnt[s[i]] = append(cnt[s[i]], j-i)
	}

	res := -1
	for _, vec := range cnt {
		lo, hi := 1, n-2
		for lo <= hi {
			mid := (lo + hi) >> 1
			count := 0
			for _, x := range vec {
				if x >= mid {
					count += x - mid + 1
				}
			}
			if count >= 3 {
				if mid > res {
					res = mid
				}
				lo = mid + 1
			} else {
				hi = mid - 1
			}
		}
	}
	return res
}

func main() {
	tests := []struct {
		s   string
		ans int
	}{
		{"aaaa", 2},
		{"abcdef", -1},
		{"abcaba", 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maximumLength(test.s), index)
	}
}
