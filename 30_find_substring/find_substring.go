/*
 * @Date: 2022-06-23
 * @LastEditors: Mengsen Wang mengsen_wang@163.com
 * @LastEditTime: 2022-06-23
 * @FilePath: /algorithm/30_find_substring/find_substring.go
 */

package main

import "reflect"

func findSubstring(s string, words []string) (ans []int) {
	ls, m, n := len(s), len(words), len(words[0])
	for i := 0; i < n && i+m*n <= ls; i++ {
		differ := map[string]int{}
		for j := 0; j < m; j++ {
			differ[s[i+j*n:i+(j+1)*n]]++
		}
		for _, word := range words {
			differ[word]--
			if differ[word] == 0 {
				delete(differ, word)
			}
		}
		for start := i; start < ls-m*n+1; start += n {
			if start != i {
				word := s[start+(m-1)*n : start+m*n]
				differ[word]++
				if differ[word] == 0 {
					delete(differ, word)
				}
				word = s[start-n : start]
				differ[word]--
				if differ[word] == 0 {
					delete(differ, word)
				}
			}
			if len(differ) == 0 {
				ans = append(ans, start)
			}
		}
	}
	return
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	assert(findSubstring("barfoothefoobarman", []string{"foo", "bar"}), []int{0, 9})
	assert(findSubstring("wordgoodgoodgoodbestword", []string{"word", "good", "best", "word"}), nil)
	assert(findSubstring("barfoofoobarthefoobarman", []string{"bar", "foo", "the"}), []int{6, 9, 12})
}
