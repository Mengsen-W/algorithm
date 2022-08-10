/*
 * @Date: 2022-08-10
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-10
 * @FilePath: /algorithm/640_solve_equation/solve_equation.go
 */

package main

import (
	"strconv"
	"unicode"
)

func solveEquation(equation string) string {
	factor, val := 0, 0
	i, n, sign := 0, len(equation), 1 // 等式左边默认系数为正
	for i < n {
		if equation[i] == '=' {
			sign = -1 // 等式右边默认系数为负
			i++
			continue
		}

		s := sign
		if equation[i] == '+' { // 去掉前面的符号
			i++
		} else if equation[i] == '-' {
			s = -s
			i++
		}

		num, valid := 0, false
		for i < n && unicode.IsDigit(rune(equation[i])) {
			valid = true
			num = num*10 + int(equation[i]-'0')
			i++
		}

		if i < n && equation[i] == 'x' { // 变量
			if valid {
				s *= num
			}
			factor += s
			i++
		} else { // 数值
			val += s * num
		}
	}

	if factor == 0 {
		if val == 0 {
			return "Infinite solutions"
		}
		return "No solution"
	}
	return "x=" + strconv.Itoa(-val/factor)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(solveEquation("x+5-3+x=6+x-2") == "x=2")
	assert(solveEquation("x=x") == "Infinite solutions")
	assert(solveEquation("2x=x") == "x=0")
}
