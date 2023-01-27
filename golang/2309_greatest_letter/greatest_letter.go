/*
 * @Date: 2023-01-27
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-27
 * @FilePath: /algorithm/golang/2309_greatest_letter/greatest_letter.go
 */

package main

import (
	"math/bits"
	"unicode"
)

func greatestLetter(s string) string {
	mask1, mask2 := 0, 0
	for _, c := range s {
		if unicode.IsLower(c) {
			mask1 |= 1 << (c - 'a')
		} else {
			mask2 |= 1 << (c - 'A')
		}
	}
	mask := mask1 & mask2
	if mask == 0 {
		return ""
	}
	return string(byte(bits.Len(uint(mask))-1) + 'A')
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		s := "lEeTcOdE"
		ans := "E"
		assert(greatestLetter(s) == ans)
	}

	{
		s := "arRAzFif"
		ans := "R"
		assert(greatestLetter(s) == ans)
	}

	{
		s := "AbCdEfGhIjK"
		ans := ""
		assert(greatestLetter(s) == ans)
	}
}
