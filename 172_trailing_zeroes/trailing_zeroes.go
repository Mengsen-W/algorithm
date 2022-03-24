/*
 * @Date: 2022-03-24 23:04:11
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-24 23:19:57
 * @FilePath: /algorithm/172_trailing_zeroes/trailing_zeroes.go
 */
package main

func trailingZeroes(n int) (ans int) {
	for n > 0 {
		n /= 5
		ans += n
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(trailingZeroes(3) == 0)
	assert(trailingZeroes(5) == 1)
	assert(trailingZeroes(0) == 0)
}
