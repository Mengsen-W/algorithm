/*
 * @Date: 2023-08-07
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-07
 * @FilePath: /algorithm/golang/344_reverse_string/reverse_string.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func reverseString(s []byte) {
	for left, right := 0, len(s)-1; left < right; left++ {
		s[left], s[right] = s[right], s[left]
		right--
	}
}

func main() {
	tests := []struct {
		s   []byte
		ans []byte
	}{
		{[]byte{'h', 'e', 'l', 'l', 'o'}, []byte{'o', 'l', 'l', 'e', 'h'}},
		{[]byte{'H', 'a', 'n', 'n', 'a', 'h'}, []byte{'h', 'a', 'n', 'n', 'a', 'H'}},
	}

	for _, item := range tests {
		reverseString(item.s)
		assert.Equal(&testing.T{}, item.ans, item.s)
	}
}
