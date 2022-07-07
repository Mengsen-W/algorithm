/*
 * @Date: 2022-07-07
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-07
 * @FilePath: /algorithm/648_replace_words/replace_words.go
 */

package main

import (
	"reflect"
	"strings"
)

func replaceWords(dictionary []string, sentence string) string {
	type trie map[rune]trie
	root := trie{}
	for _, s := range dictionary {
		cur := root
		for _, c := range s {
			if cur[c] == nil {
				cur[c] = trie{}
			}
			cur = cur[c]
		}
		cur['#'] = trie{}
	}

	words := strings.Split(sentence, " ")
	for i, word := range words {
		cur := root
		for j, c := range word {
			if cur['#'] != nil {
				words[i] = word[:j]
				break
			}
			if cur[c] == nil {
				break
			}
			cur = cur[c]
		}
	}
	return strings.Join(words, " ")
}

func main() {
	assert := func(a, b string) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	{
		dictionary := []string{"cat", "bat", "rat"}
		sentence := "the cattle was rattled by the battery"
		ans := "the cat was rat by the bat"
		assert(replaceWords(dictionary, sentence), ans)
	}

	{
		dictionary := []string{"a", "b", "c"}
		sentence := "aadsfasf absbs bbab cadsfafs"
		ans := "a a b c"
		assert(replaceWords(dictionary, sentence), ans)
	}
}
