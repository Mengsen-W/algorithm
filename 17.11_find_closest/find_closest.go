/*
 * @Date: 2022-05-28 10:18:32
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-28 10:24:05
 * @FilePath: /algorithm/17.11_find_closest/find_closest.go
 */

package main

func findClosest(words []string, word1, word2 string) int {
	abs := func(x int) int {
		if x < 0 {
			return -x
		}
		return x
	}

	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}

	ans := len(words)
	index1, index2 := -1, -1
	for i, word := range words {
		if word == word1 {
			index1 = i
		} else if word == word2 {
			index2 = i
		}
		if index1 >= 0 && index2 >= 0 {
			ans = min(ans, abs(index1-index2))
		}
	}
	return ans
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(findClosest([]string{"I", "am", "a", "student", "from", "a", "university", "in", "a", "city"}, "a", "student") == 1)
}
