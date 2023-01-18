/*
 * @Date: 2021-11-04 00:52:59
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-04 00:57:21
 */

package main

func isPerfectSquare(num int) bool {
	x0 := float64(num)
	for {
		x1 := (x0 + float64(num)/x0) / 2
		if x0-x1 < 1e-6 {
			x := int(x0)
			return x*x == num
		}
		x0 = x1
	}
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("assertion failed")
		}
	}
	assert(isPerfectSquare(16) == true)
	assert(isPerfectSquare(14) == false)
}
