/*
 * @Date: 2021-08-13 11:39:14
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-13 11:46:13
 */

package main

import (
	"math"
)

func countDigitOne(n int) int {
	num, s, i := n, 0, 1
	for num != 0 {
		if num%10 == 0 {
			s = s + (num/10)*i
		}
		if num%10 == 1 {
			s = s + (num/10)*i + (n % i) + 1
		}
		if num%10 > 1 {
			s = s + int(math.Ceil(float64(num)/10.0))*i
		}
		num = num / 10
		i = i * 10
	}
	return s
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(countDigitOne(13) == 6)
	assert(countDigitOne(0) == 0)
	assert(countDigitOne(1000000000) == 900000001)
}
