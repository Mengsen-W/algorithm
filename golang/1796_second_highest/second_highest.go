/*
 * @Date: 2022-12-03
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-03
 * @FilePath: /algorithm/1796_second_highest/second_highest.go
 */

package main

import "unicode"

func secondHighest(s string) int {
	first, second := -1, -1
	for _, c := range s {
		if unicode.IsDigit(c) {
			num := int(c - '0')
			if num > first {
				second = first
				first = num
			} else if second < num && num < first {
				second = num
			}
		}
	}
	return second
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(secondHighest("dfa12321afd") == 2)
	assert(secondHighest("abc1111") == -1)
}
