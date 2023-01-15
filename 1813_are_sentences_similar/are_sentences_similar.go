/*
 * @Date: 2023-01-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-16
 * @FilePath: /algorithm/1813_are_sentences_similar/are_sentences_similar.go
 */

package main

import "strings"

func areSentencesSimilar(sentence1, sentence2 string) bool {
	words1 := strings.Split(sentence1, " ")
	words2 := strings.Split(sentence2, " ")
	i, n := 0, len(words1)
	j, m := 0, len(words2)
	for i < n && i < m && words1[i] == words2[i] {
		i++
	}
	for j < n-i && j < m-i && words1[n-j-1] == words2[m-j-1] {
		j++
	}

	return i+j == func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}(n, m)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		sentence1 := "My name is Haley"
		sentence2 := "My Haley"
		assert(areSentencesSimilar(sentence1, sentence2))
	}

	{
		sentence1 := "of"
		sentence2 := "A lot of words"
		assert(!areSentencesSimilar(sentence1, sentence2))
	}

	{
		sentence1 := "Eating right now"
		sentence2 := "Eating"
		assert(areSentencesSimilar(sentence1, sentence2))
	}

	{
		sentence1 := "Luky"
		sentence2 := "Lucccky"
		assert(!areSentencesSimilar(sentence1, sentence2))
	}
}
