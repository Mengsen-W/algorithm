/*
 * @Date: 2023-03-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-29
 * @FilePath: /algorithm/golang/1641_count_vowel_strings/count_vowel_strings.go
 */

// Package main ...
package main

func countVowelStrings(n int) int {
	return (n + 1) * (n + 2) * (n + 3) * (n + 4) / 24
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(countVowelStrings(1) == 5)
	assert(countVowelStrings(2) == 15)
}
