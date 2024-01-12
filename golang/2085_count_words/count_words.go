/*
 * @Date: 2024-01-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-12
 * @FilePath: /algorithm/golang/2085_count_words/count_words.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countWords(words1 []string, words2 []string) int {
	// 统计字符串出现频率
	freq1 := make(map[string]int)
	freq2 := make(map[string]int)
	for _, w := range words1 {
		freq1[w]++
	}
	for _, w := range words2 {
		freq2[w]++
	}

	// 遍历 words1 出现的字符串并检查个数
	res := 0
	for w, cnt1 := range freq1 {
		if cnt1 == 1 && freq2[w] == 1 {
			res++
		}
	}
	return res
}

func main() {
	tests := []struct {
		words1 []string
		words2 []string
		ans    int
	}{
		{[]string{"leetcode", "is", "amazing", "as", "is"}, []string{"amazing", "leetcode", "is"}, 2},
		{[]string{"b", "bb", "bbb"}, []string{"a", "aa", "aaa"}, 0},
		{[]string{"a", "ab"}, []string{"a", "a", "a", "ab"}, 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countWords(test.words1, test.words2), index)
	}
}
