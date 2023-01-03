/*
 * @Date: 2023-01-03
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2023-01-03
 * @FilePath: /algorithm/2024_are_numbers_ascending/are_numbers_ascending.go
 */

package main

import "unicode"

func areNumbersAscending(s string) bool {
	pre, i := 0, 0
	for i < len(s) {
		if unicode.IsDigit(rune(s[i])) {
			cur := 0
			for i < len(s) && unicode.IsDigit(rune(s[i])) {
				cur = cur*10 + int(s[i]-'0')
				i++
			}
			if cur <= pre {
				return false
			}
			pre = cur
		} else {
			i++
		}
	}
	return true
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		s := "1 box has 3 blue 4 red 6 green and 12 yellow marbles"
		ans := true
		assert(areNumbersAscending(s) == ans)
	}

	{
		s := "hello world 5 x 5"
		ans := false
		assert(areNumbersAscending(s) == ans)
	}

	{
		s := "sunset is at 7 51 pm overnight lows will be in the low 50 and 60 s"
		ans := false
		assert(areNumbersAscending(s) == ans)
	}

	{
		s := "4 5 11 26"
		ans := true
		assert(areNumbersAscending(s) == ans)
	}
}
