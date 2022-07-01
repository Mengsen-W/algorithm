/*
 * @Date: 2022-07-01
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-01
 * @FilePath: /algorithm/241_diff_ways_to_compute/diff_ways_to_compute.go
 */

package main

import (
	"reflect"
	"unicode"
)

const addition, subtraction, multiplication = -1, -2, -3

func diffWaysToCompute(expression string) []int {
	ops := []int{}
	for i, n := 0, len(expression); i < n; {
		if unicode.IsDigit(rune(expression[i])) {
			x := 0
			for ; i < n && unicode.IsDigit(rune(expression[i])); i++ {
				x = x*10 + int(expression[i]-'0')
			}
			ops = append(ops, x)
		} else {
			if expression[i] == '+' {
				ops = append(ops, addition)
			} else if expression[i] == '-' {
				ops = append(ops, subtraction)
			} else {
				ops = append(ops, multiplication)
			}
			i++
		}
	}

	n := len(ops)
	dp := make([][][]int, n)
	for i, x := range ops {
		dp[i] = make([][]int, n)
		dp[i][i] = []int{x}
	}
	for sz := 3; sz <= n; sz++ {
		for l, r := 0, sz-1; r < n; l += 2 {
			for k := l + 1; k < r; k += 2 {
				for _, x := range dp[l][k-1] {
					for _, y := range dp[k+1][r] {
						if ops[k] == addition {
							dp[l][r] = append(dp[l][r], x+y)
						} else if ops[k] == subtraction {
							dp[l][r] = append(dp[l][r], x-y)
						} else {
							dp[l][r] = append(dp[l][r], x*y)
						}
					}
				}
			}
			r += 2
		}
	}
	return dp[0][n-1]
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	assert(diffWaysToCompute("2-1-1"), []int{2, 0})
	assert(diffWaysToCompute("2*3-4*5"), []int{-34, -10, -14, -10, 10})
}
