// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func robotWithString(s string) string {
	cnt := make([]int, 26)
	for _, c := range s {
		cnt[c-'a']++
	}

	stack := []rune{}
	res := []rune{}
	minCharacter := 'a'
	for _, c := range s {
		stack = append(stack, c)
		cnt[c-'a']--
		for minCharacter != 'z' && cnt[minCharacter-'a'] == 0 {
			minCharacter++
		}
		for len(stack) > 0 && stack[len(stack)-1] <= minCharacter {
			res = append(res, stack[len(stack)-1])
			stack = stack[:len(stack)-1]
		}
	}

	return string(res)
}

func main() {
	tests := []struct {
		s   string
		ans string
	}{
		{"zza", "azz"},
		{"bac", "abc"},
		{"bdda", "addb"},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, robotWithString(test.s), index)
	}
}
