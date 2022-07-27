/*
 * @Date: 2022-07-27
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-07-27
 * @FilePath: /algorithm/592_fraction_addition/fraction_addition.go
 */

package main

import (
	"fmt"
	"unicode"
)

func fractionAddition(expression string) string {
	denominator, numerator := 0, 1 // 分子，分母
	for i, n := 0, len(expression); i < n; {
		// 读取分子
		denominator1, sign := 0, 1
		if expression[i] == '-' || expression[i] == '+' {
			if expression[i] == '-' {
				sign = -1
			}
			i++
		}
		for i < n && unicode.IsDigit(rune(expression[i])) {
			denominator1 = denominator1*10 + int(expression[i]-'0')
			i++
		}
		denominator1 = sign * denominator1
		i++

		// 读取分母
		numerator1 := 0
		for i < n && unicode.IsDigit(rune(expression[i])) {
			numerator1 = numerator1*10 + int(expression[i]-'0')
			i++
		}

		denominator = denominator*numerator1 + denominator1*numerator
		numerator *= numerator1
	}
	if denominator == 0 {
		return "0/1"
	}
	g := func(a, b int) int {
		for a != 0 {
			a, b = b%a, a
		}
		return b
	}(func(x int) int {
		if x < 0 {
			return -x
		}
		return x
	}(denominator), numerator)
	return fmt.Sprintf("%d/%d", denominator/g, numerator/g)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		expression := "-1/2+1/2"
		ans := "0/1"
		assert(fractionAddition(expression) == ans)
	}
	{
		expression := "-1/2+1/2+1/3"
		ans := "1/3"
		assert(fractionAddition(expression) == ans)
	}
	{
		expression := "1/3-1/2"
		ans := "-1/6"
		assert(fractionAddition(expression) == ans)
	}
}
