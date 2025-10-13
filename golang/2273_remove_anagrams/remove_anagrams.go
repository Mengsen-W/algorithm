// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func removeAnagrams(words []string) []string {
	res := []string{words[0]} // 结果数组
	n := len(words)

	// 判断两个单词是否为字母异位词
	compare := func(word1, word2 string) bool {
		freq := make([]int, 26)
		for _, ch := range word1 {
			freq[ch-'a']++
		}
		for _, ch := range word2 {
			freq[ch-'a']--
		}
		for _, x := range freq {
			if x != 0 {
				return false
			}
		}
		return true
	}

	for i := 1; i < n; i++ {
		if !compare(words[i], words[i-1]) {
			res = append(res, words[i])
		}
	}
	return res
}

func main() {
	tests := []struct {
		words []string
		ans   []string
	}{
		{[]string{"abba", "baba", "bbaa", "cd", "cd"}, []string{"abba", "cd"}},
		{[]string{"a", "b", "c", "d", "e"}, []string{"a", "b", "c", "d", "e"}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, removeAnagrams(test.words), index)
	}
}
