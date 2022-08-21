/*
 * @Date: 2022-08-21
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-21
 * @FilePath: /algorithm/1455_is_prefix_of_word/is_prefix_of_word.go
 */

package main

import "strings"

func isPrefixOfWord(sentence, searchWord string) int {
	for i, index, n := 0, 1, len(sentence); i < n; i++ {
		start := i
		for i < n && sentence[i] != ' ' {
			i++
		}
		end := i
		if strings.HasPrefix(sentence[start:end], searchWord) {
			return index
		}
		index++
	}
	return -1
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		sentence := "i love eating burger"
		searchWord := "burg"
		ans := 4
		assert(isPrefixOfWord(sentence, searchWord) == ans)
	}
	{
		sentence := "this problem is an easy problem"
		searchWord := "pro"
		ans := 2
		assert(isPrefixOfWord(sentence, searchWord) == ans)
	}
	{
		sentence := "i am tired"
		searchWord := "you"
		ans := -1
		assert(isPrefixOfWord(sentence, searchWord) == ans)
	}
}
