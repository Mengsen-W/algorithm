/*
 * @Date: 2021-09-23 08:48:55
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-23 08:50:03
 */

package main

func isPowerOfThree(n int) bool {
	for n > 0 && n%3 == 0 {
		n /= 3
	}
	return n == 1
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(isPowerOfThree(27))
	assert(!isPowerOfThree(0))
	assert(isPowerOfThree(9))
	assert(!isPowerOfThree(45))
}
