// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func clearStars(s string) string {
	cnt := make([][]int, 26)
	for i := range cnt {
		cnt[i] = make([]int, 0)
	}
	arr := []rune(s)
	for i, c := range arr {
		if c != '*' {
			cnt[c-'a'] = append(cnt[c-'a'], i)
		} else {
			for j := 0; j < 26; j++ {
				if len(cnt[j]) > 0 {
					last := len(cnt[j]) - 1
					arr[cnt[j][last]] = '*'
					cnt[j] = cnt[j][:last]
					break
				}
			}
		}
	}
	var ans []rune
	for _, c := range arr {
		if c != '*' {
			ans = append(ans, c)
		}
	}
	return string(ans)
}

func main() {
	tests := []struct {
		s   string
		ans string
	}{
		{"aaba*", "aab"},
		{"abc", "abc"},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, clearStars(test.s), index)
	}
}
