// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func smallestBeautifulString(s string, k int) string {
	for i := len(s) - 1; i >= 0; i-- {
		blockedCharacters := make(map[byte]bool)
		for j := 1; j < 3; j++ {
			if i-j >= 0 {
				blockedCharacters[s[i-j]] = true
			}
		}
		for j := 1; j < 4; j++ {
			if int(s[i]-'a')+j+1 <= k && !blockedCharacters[s[i]+byte(j)] {
				return generate(s, i, j)
			}
		}
	}
	return ""
}

func generate(s string, idx int, offset int) string {
	res := []byte(s)
	res[idx] += byte(offset)
	for i := idx + 1; i < len(s); i++ {
		blockedCharacters := make(map[byte]bool)
		for j := 1; j < 3; j++ {
			if i-j >= 0 {
				blockedCharacters[res[i-j]] = true
			}
		}
		for c := byte('a'); c <= byte('c'); c++ {
			if !blockedCharacters[c] {
				res[i] = c
				break
			}
		}
	}
	return string(res)
}

func main() {
	tests := []struct {
		s   string
		k   int
		ans string
	}{
		{"abcz", 26, "abda"},
		{"dc", 4, ""},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, smallestBeautifulString(test.s, test.k), index)
	}
}
