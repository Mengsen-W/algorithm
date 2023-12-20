/*
 * @Date: 2023-12-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-20
 * @FilePath: /algorithm/golang/2828_is_acronym/is_acronym.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func isAcronym(words []string, s string) bool {
	if len(words) != len(s) {
		return false
	}
	for i := 0; i < len(s); i++ {
		if words[i][0] != s[i] {
			return false
		}
	}
	return true
}

func main() {
	tests := []struct {
		words []string
		s     string
		ans   bool
	}{
		{[]string{"alice", "bob", "charlie"}, "abc", true},
		{[]string{"an", "apple"}, "a", false},
		{[]string{"never", "gonna", "give", "up", "on", "you"}, "ngguoy", true},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, isAcronym(test.words, test.s), index)
	}
}
