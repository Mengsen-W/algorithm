// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func longestBalanced(s string) int {
	n := len(s)
	res := 0

	for i := 0; i < n; i++ {
		cnt := make([]int, 26)

		for j := i; j < n; j++ {
			c := s[j] - 'a'
			cnt[c]++
			flag := true

			for _, x := range cnt {
				if x > 0 && x != cnt[c] {
					flag = false
					break
				}
			}

			if flag && (j-i+1) > res {
				res = j - i + 1
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
		{"abbac", 4},
		{"zzabccy", 4},
		{"aba", 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, longestBalanced(test.s), index)
	}
}
