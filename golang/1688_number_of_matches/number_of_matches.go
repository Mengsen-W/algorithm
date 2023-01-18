/*
 * @Date: 2022-01-25 00:27:05
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-25 00:41:04
 */

package main

func numberOfMatches1(n int) int {
	ans := 0
	for n > 1 {
		if n%2 == 0 {
			ans += n / 2
			n /= 2
		} else {
			ans += (n - 1) / 2
			n = (n-1)/2 + 1
		}
	}
	return ans
}

func numberOfMatches2(n int) int {
	return n - 1
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(numberOfMatches1(7) == 6)
	assert(numberOfMatches1(14) == 13)
	assert(numberOfMatches2(7) == 6)
	assert(numberOfMatches2(14) == 13)
}
