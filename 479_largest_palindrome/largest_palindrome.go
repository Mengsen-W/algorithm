/*
 * @Date: 2022-04-16 16:10:57
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-16 16:28:45
 * @FilePath: /algorithm/479_largest_palindrome/largest_palindrome.go
 */

package main

import "math"

func largestPalindrome(n int) int {
	if n == 1 {
		return 9
	}
	upper := int(math.Pow10(n)) - 1
	for left := upper; ; left-- { // 枚举回文数的左半部分
		p := left
		for x := left; x > 0; x /= 10 {
			p = p*10 + x%10 // 翻转左半部分到其自身末尾，构造回文数 p
		}
		for x := upper; x*x >= p; x-- {
			if p%x == 0 { // x 是 p 的因子
				return p % 1337
			}
		}
	}
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(largestPalindrome(2) == 987)
	assert(largestPalindrome(1) == 9)
}
