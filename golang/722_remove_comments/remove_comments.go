/*
 * @Date: 2023-08-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-03
 * @FilePath: /algorithm/golang/722_remove_comments/remove_comments.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func removeComments(source []string) (ans []string) {
	t := []byte{}
	blockComment := false
	for _, s := range source {
		m := len(s)
		for i := 0; i < m; i++ {
			if blockComment {
				if i+1 < m && s[i] == '*' && s[i+1] == '/' {
					blockComment = false
					i++
				}
			} else {
				if i+1 < m && s[i] == '/' && s[i+1] == '*' {
					blockComment = true
					i++
				} else if i+1 < m && s[i] == '/' && s[i+1] == '/' {
					break
				} else {
					t = append(t, s[i])
				}
			}
		}
		if !blockComment && len(t) > 0 {
			ans = append(ans, string(t))
			t = []byte{}
		}
	}
	return
}

func main() {
	tests := []struct {
		source []string
		ans    []string
	}{
		{
			[]string{
				"/*Test program */", "int main()", "{ ", "  // variable declaration ", "int a, b, c;", "/* This is a test",
				"   multiline  ", "   comment for ", "   testing */", "a = b + c;", "}",
			},
			[]string{"int main()", "{ ", "  ", "int a, b, c;", "a = b + c;", "}"},
		},
		{[]string{"a/*comment", "line", "more_comment*/b"}, []string{"ab"}},
	}

	for _, item := range tests {
		assert.Equal(&testing.T{}, removeComments(item.source), item.ans)
	}
}
