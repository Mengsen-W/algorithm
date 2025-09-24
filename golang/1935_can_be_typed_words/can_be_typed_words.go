// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func canBeTypedWords(text string, brokenLetters string) int {
	broken := make(map[rune]bool) // 无法输入的字符集合
	for _, ch := range brokenLetters {
		broken[ch] = true
	}
	res := 0     // 可以完全输入的单词数目
	flag := true // 当前字符所在单词是否可被完全输入
	for _, ch := range text {
		if ch == ' ' {
			// 当前字符为空格，检查上一个单词状态，更新数目并初始化 flag
			if flag {
				res++
			}
			flag = true
		} else if broken[ch] {
			// 当前字符不可被输入，所在单词无法被完全输入，更新 flag
			flag = false
		}
	}
	// 判断最后一个单词状态并更新数目
	if flag {
		res++
	}
	return res
}

func main() {
	tests := []struct {
		text          string
		brokenLetters string
		ans           int
	}{
		{"hello world", "ad", 1},
		{"leet code", "lt", 1},
		{"leet code", "e", 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, canBeTypedWords(test.text, test.brokenLetters), index)
	}
}
