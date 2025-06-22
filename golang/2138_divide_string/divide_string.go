// Package main ...
package main

import (
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func divideString(s string, k int, fill byte) []string {
	var res []string // 分组后的字符串
	n := len(s)
	curr := 0 // 每个分组的起始下标
	// 拆分字符串
	for curr < n {
		end := curr + k
		if end > n {
			end = n
		}
		res = append(res, s[curr:end])
		curr += k
	}
	// 尝试填充最后一组
	last := res[len(res)-1]
	if len(last) < k {
		last += strings.Repeat(string(fill), k-len(last))
		res[len(res)-1] = last
	}
	return res
}

func main() {
	tests := []struct {
		s    string
		k    int
		fill byte
		ans  []string
	}{
		{"abcdefghi", 3, 'x', []string{"abc", "def", "ghi"}},
		{"abcdefghij", 3, 'x', []string{"abc", "def", "ghi", "jxx"}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, divideString(test.s, test.k, test.fill), index)
	}
}
