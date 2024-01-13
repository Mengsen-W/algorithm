/*
 * @Date: 2024-01-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-13
 * @FilePath: /algorithm/golang/2182_repeat_limited_string/repeat_limited_string.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func repeatLimitedString(s string, repeatLimit int) string {
	const N = 26
	count := make([]int, 26)
	for _, c := range s {
		count[c-'a']++
	}
	var ret []byte
	var m int
	for i, j := N-1, N-2; i >= 0 && j >= 0; {
		switch {
		case count[i] == 0: // 当前字符已经填完，填入后面的字符，重置 m
			m, i = 0, i-1
		case m < repeatLimit: // 当前字符未超过限制
			count[i]--
			ret = append(ret, 'a'+byte(i))
			m++
		case j >= i || count[j] == 0: // 当前字符已经超过限制，查找可填入的其他字符
			j--
		default: // 当前字符已经超过限制，填入其他字符，并且重置 m
			count[j]--
			ret = append(ret, 'a'+byte(j))
			m = 0
		}
	}
	return string(ret)
}

func main() {
	tests := []struct {
		s           string
		repeatLimit int
		ans         string
	}{
		{"cczazcc", 3, "zzcccac"},
		{"aababab", 2, "bbabaa"},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, repeatLimitedString(test.s, test.repeatLimit), index)
	}
}
