/*
 * @Date: 2024-01-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-20
 * @FilePath: /algorithm/golang/2788_split_words_by_separator/split_words_by_separator.go
 */

// Package main ...
package main

import (
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func splitWordsBySeparator(words []string, separator byte) []string {
	res := []string{}
	for _, word := range words {
		subs := strings.Split(word, string([]byte{separator}))
		for _, sub := range subs {
			if len(sub) > 0 {
				res = append(res, sub)
			}
		}
	}
	return res
}

func main() {
	tests := []struct {
		words     []string
		separator byte
		ans       []string
	}{
		{[]string{"one.two.three", "four.five", "six"}, '.', []string{"one", "two", "three", "four", "five", "six"}},
		{[]string{"$easy$", "$problem$"}, '$', []string{"easy", "problem"}},
		{[]string{"|||"}, '|', []string{}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, splitWordsBySeparator(test.words, test.separator), index)
	}
}
