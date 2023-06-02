/*
 * @Date: 2023-06-02
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-02
 * @FilePath: /algorithm/golang/2559_vowel_strings/vowel_strings.go
 */

// Package main ...
package main

import "reflect"

func vowelStrings(words []string, queries [][]int) []int {
	n := len(words)
	prefixSums := make([]int, n+1)
	for i := 0; i < n; i++ {
		value := 0
		if isVowelString(words[i]) {
			value = 1
		}
		prefixSums[i+1] = prefixSums[i] + value
	}
	ans := make([]int, len(queries))
	for i := 0; i < len(queries); i++ {
		start := queries[i][0]
		end := queries[i][1]
		ans[i] = prefixSums[end+1] - prefixSums[start]
	}
	return ans
}

func isVowelString(word string) bool {
	return isVowelLetter(word[0]) && isVowelLetter(word[len(word)-1])
}

func isVowelLetter(c byte) bool {
	return c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		words := []string{"aba", "bcb", "ece", "aa", "e"}
		queries := [][]int{{0, 2}, {1, 4}, {1, 1}}
		ans := []int{2, 3, 0}
		assert(vowelStrings(words, queries), ans)
	}

	{
		words := []string{"a", "e", "i"}
		queries := [][]int{{0, 2}, {0, 1}, {2, 2}}
		ans := []int{3, 2, 1}
		assert(vowelStrings(words, queries), ans)
	}
}
