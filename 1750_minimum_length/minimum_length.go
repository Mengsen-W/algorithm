/*
 * @Date: 2022-12-28
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-28
 * @FilePath: /algorithm/1750_minimum_length/minimum_length.go
 */

package main

func minimumLength(s string) int {
	left, right := 0, len(s)-1
	for left < right && s[left] == s[right] {
		c := s[left]
		for left <= right && s[left] == c {
			left++
		}
		for right >= left && s[right] == c {
			right--
		}
	}
	return right - left + 1
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		s := "ca"
		ans := 2
		assert(minimumLength(s) == ans)
	}

	{
		s := "cabaabac"
		ans := 0
		assert(minimumLength(s) == ans)
	}

	{
		s := "aabccabba"
		ans := 3
		assert(minimumLength(s) == ans)
	}
}
