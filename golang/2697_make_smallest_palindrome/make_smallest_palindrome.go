/*
 * @Date: 2023-12-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-13
 * @FilePath: /algorithm/golang/2697_make_smallest_palindrome/make_smallest_palindrome.go
 */

package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func makeSmallestPalindrome(s string) string {
	left, right := 0, len(s)-1
	t := []byte(s)
	for left < right {
		if s[left] != s[right] {
			t[left] = min(s[left], s[right])
			t[right] = t[left]
		}
		left++
		right--
	}
	return string(t)
}

func main() {
	tests := []struct {
		s   string
		ans string
	}{
		{"egcfe", "efcfe"},
		{"abcd", "abba"},
		{"seven", "neven"},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, makeSmallestPalindrome(test.s), index)
	}
}
