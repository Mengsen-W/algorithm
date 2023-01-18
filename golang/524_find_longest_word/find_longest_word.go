/*
 * @Date: 2021-09-14 08:52:21
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-14 09:21:14
 */

package main

func findLongestWord(s string, dictionary []string) string {
	canFormByDeleting := func(word, str string) bool {
		word_i, str_i := 0, 0
		word_size, str_size := len(word), len(str)
		for word_i < word_size && str_i < str_size {
			if word[word_i] == str[str_i] {
				word_i++
			}
			str_i++
		}
		return word_i == word_size
	}
	res := ""
	for _, str := range dictionary {
		if canFormByDeleting(str, s) {
			if len(str) > len(res) || len(str) == len(res) && str < res {
				res = str
			}
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
		s := "abpcplea"
		dictionary := []string{"ale", "apple", "monkey", "plea"}
		ans := "apple"
		assert(findLongestWord(s, dictionary) == ans)
	}
	{
		s := "abpcplea"
		dictionary := []string{"a", "b", "c"}
		ans := "a"
		assert(findLongestWord(s, dictionary) == ans)
	}
}
