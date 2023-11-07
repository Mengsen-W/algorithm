/*
 * @Date: 2023-11-07
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-07
 * @FilePath: /algorithm/golang/2586_vowel_strings/vowel_strings.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func vowelStrings(words []string, left int, right int) int {
	vowels := map[byte]struct{}{'a': {}, 'e': {}, 'i': {}, 'o': {}, 'u': {}}
	ans := 0
	for _, word := range words[left : right+1] {
		if _, ok1 := vowels[word[0]]; ok1 {
			if _, ok2 := vowels[word[len(word)-1]]; ok2 {
				ans++
			}
		}
	}
	return ans
}

func main() {
	tests := []struct {
		words []string
		left  int
		right int
		ans   int
	}{
		{[]string{"are", "amy", "u"}, 0, 2, 2},
		{[]string{"hey", "aeo", "mu", "ooo", "artro"}, 1, 4, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, vowelStrings(test.words, test.left, test.right), index)
	}
}
