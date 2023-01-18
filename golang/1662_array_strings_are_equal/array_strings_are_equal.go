/*
 * @Date: 2022-11-01
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-01
 * @FilePath: /algorithm/1662_array_strings_are_equal/array_strings_are_equal.go
 */

package main

func arrayStringsAreEqual(word1, word2 []string) bool {
	var p1, p2, i, j int
	for p1 < len(word1) && p2 < len(word2) {
		if word1[p1][i] != word2[p2][j] {
			return false
		}
		i++
		if i == len(word1[p1]) {
			p1++
			i = 0
		}

		j++
		if j == len(word2[p2]) {
			p2++
			j = 0
		}
	}
	return p1 == len(word1) && p2 == len(word2)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		word1 := []string{"ab", "c"}
		word2 := []string{"a", "bc"}
		assert(arrayStringsAreEqual(word1, word2))
	}

	{
		word1 := []string{"a", "cb"}
		word2 := []string{"ab", "c"}
		assert(!arrayStringsAreEqual(word1, word2))
	}

	{
		word1 := []string{"abc", "d", "defg"}
		word2 := []string{"abcddefg"}
		assert(arrayStringsAreEqual(word1, word2))
	}
}
