/*
 * @Date: 2022-09-17
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-17
 * @FilePath: /algorithm/1624_max_length_between_equal_characters/max_length_between_equal_characters.go
 */

package main

func maxLengthBetweenEqualCharacters(s string) int {
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}
	ans := -1
	firstIndex := [26]int{}
	for i := range firstIndex {
		firstIndex[i] = -1
	}
	for i, c := range s {
		c -= 'a'
		if firstIndex[c] < 0 {
			firstIndex[c] = i
		} else {
			ans = max(ans, i-firstIndex[c]-1)
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
	assert(maxLengthBetweenEqualCharacters("aa") == 0)
	assert(maxLengthBetweenEqualCharacters("abca") == 2)
	assert(maxLengthBetweenEqualCharacters("cbzxy") == -1)
	assert(maxLengthBetweenEqualCharacters("cabbac") == 4)
}
