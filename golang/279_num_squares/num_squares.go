/*
 * @Date: 2021-06-11 08:30:32
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-11 08:39:39
 */

package main

import "math"

func numSquares(n int) int {
	// 判断是否为完全平方数
	isPerfectSquare := func(x int) bool {
		y := int(math.Sqrt(float64(x)))
		return y*y == x
	}
	// 判断是否能表示为 4^k*(8m+7)
	checkAnswer4 := func(x int) bool {
		for x%4 == 0 {
			x /= 4
		}
		return x%8 == 7
	}
	if isPerfectSquare(n) {
		return 1
	}
	if checkAnswer4(n) {
		return 4
	}
	for i := 1; i*i <= n; i++ {
		j := n - i*i
		if isPerfectSquare(j) {
			return 2
		}
	}
	return 3
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed!")
		}
	}
	assert(numSquares(12) == 3)
	assert(numSquares(13) == 2)
}
