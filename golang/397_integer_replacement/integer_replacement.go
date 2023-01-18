/*
 * @Date: 2021-11-19 00:29:35
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-19 00:42:49
 */

package main

func integerReplacement(n int) (ans int) {
	for n != 1 {
		switch {
		case n%2 == 0:
			ans++
			n /= 2
		case n%4 == 1:
			ans += 2
			n /= 2
		case n == 3:
			ans += 2
			n = 1
		default:
			ans += 2
			n = n/2 + 1
		}
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(integerReplacement(8) == 3)
	assert(integerReplacement(7) == 4)
	assert(integerReplacement(4) == 2)
}
