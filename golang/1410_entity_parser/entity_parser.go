/*
 * @Date: 2023-11-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-23
 * @FilePath: /algorithm/golang/1410_entity_parser/entity_parser.go
 */

// Package main ...
package main

import (
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func entityParser(text string) string {
	entityMap := map[string]string{
		"&quot;":  "\"",
		"&apos;":  "'",
		"&gt;":    ">",
		"&lt;":    "<",
		"&frasl;": "/",
		"&amp;":   "&",
	}

	i := 0
	n := len(text)
	res := make([]string, 0)
	for i < n {
		isEntity := false
		if text[i] == '&' {
			for k, v := range entityMap {
				if i+len(k) <= n && text[i:i+len(k)] == k {
					res = append(res, v)
					isEntity = true
					i += len(k)
					break
				}
			}
		}
		if !isEntity {
			res = append(res, text[i:i+1])
			i++
		}
	}
	return strings.Join(res, "")
}

func main() {
	tests := []struct {
		text string
		ans  string
	}{
		{"&amp; is an HTML entity but &ambassador; is not.", "& is an HTML entity but &ambassador; is not."},
		{"and I quote: &quot;...&quot;", "and I quote: \"...\""},
		{"Stay home! Practice on Leetcode :)", "Stay home! Practice on Leetcode :)"},
		{"x &gt; y &amp;&amp; x &lt; y is always false", "x > y && x < y is always false"},
		{"leetcode.com&frasl;problemset&frasl;all", "leetcode.com/problemset/all"},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, entityParser(test.text), index)
	}
}
