/*
 * @Date: 2023-04-27
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-27
 * @FilePath: /algorithm/golang/1048_longest_str_chain/longest_str_chain.go
 */

// Package main ...
package main

import "sort"

func longestStrChain(words []string) int {
	cnt := map[string]int{}
	sort.Slice(words, func(i, j int) bool { return len(words[i]) < len(words[j]) })
	res := 0
	for _, word := range words {
		cnt[word] = 1
		for i := range word {
			prev := word[:i] + word[i+1:]
			if j := cnt[prev] + 1; j > cnt[word] {
				cnt[word] = j
			}
		}
		if cnt[word] > res {
			res = cnt[word]
		}
	}
	return res
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		words := []string{"a", "b", "ba", "bca", "bda", "bdca"}
		ans := 4
		assert(longestStrChain(words) == ans)
	}

	{
		words := []string{"xbc", "pcxbcf", "xb", "cxbc", "pcxbc"}
		ans := 5
		assert(longestStrChain(words) == ans)
	}

	{
		words := []string{"abcd", "dbqca"}
		ans := 1
		assert(longestStrChain(words) == ans)
	}
}
