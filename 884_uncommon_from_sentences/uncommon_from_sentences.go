/*
 * @Date: 2022-01-30 00:59:42
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-30 01:10:21
 */

package main

import (
	"reflect"
	"strings"
)

func uncommonFromSentences(s1 string, s2 string) []string {
	freq := make(map[string]int)

	insert := func(s string) {
		words := strings.Split(s, " ")
		for _, word := range words {
			freq[word]++
		}
	}

	insert(s1)
	insert(s2)

	ans := []string{}
	for word, occ := range freq {
		if occ == 1 {
			ans = append(ans, word)
		}
	}
	return ans
}

func main() {
	assert := func(a, b []string) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	assert(uncommonFromSentences("this apple is sweet", "this apple is sour"), []string{"sweet", "sour"})
	assert(uncommonFromSentences("apple apple", "banana"), []string{"banana"})
}
