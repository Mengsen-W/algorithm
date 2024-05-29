// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximumLength(s string) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}

	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}
	ans := -1
	length := len(s)

	chs := make([][]int, 26)
	cnt := 0
	for i := 0; i < length; i++ {
		cnt++
		if i+1 == length || s[i] != s[i+1] {
			ch := int(s[i] - 'a')
			chs[ch] = append(chs[ch], cnt)
			cnt = 0

			for j := len(chs[ch]) - 1; j > 0; j-- {
				if chs[ch][j] > chs[ch][j-1] {
					tmp := chs[ch][j-1]
					chs[ch][j-1] = chs[ch][j]
					chs[ch][j] = tmp
				} else {
					break
				}
			}

			if len(chs[ch]) > 3 {
				chs[ch] = chs[ch][:len(chs[ch])-1]
			}
		}
	}

	for i := 0; i < 26; i++ {
		if len(chs[i]) > 0 && chs[i][0] > 2 {
			ans = max(ans, chs[i][0]-2)
		}
		if len(chs[i]) > 1 && chs[i][0] > 1 {
			ans = max(ans, min(chs[i][0]-1, chs[i][1]))
		}
		if len(chs[i]) > 2 {
			ans = max(ans, chs[i][2])
		}
	}

	return ans
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
