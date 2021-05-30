/*
 * @Date: 2021-05-30 09:22:59
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-30 09:30:05
 */

package main

func isPowerOfTwo_1(n int) bool {
	return n > 0 && n&(n-1) == 0
}

func isPowerOfTwo_2(n int) bool {
	return n > 0 && n&-n == n
}
func isPowerOfTwo_3(n int) bool {
	const big = 1 << 30
	return n > 0 && big%n == 0
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed!")
		}
	}
	assert(isPowerOfTwo_1(1))
	assert(isPowerOfTwo_2(1))
	assert(isPowerOfTwo_3(1))
	assert(isPowerOfTwo_1(16))
	assert(isPowerOfTwo_2(16))
	assert(isPowerOfTwo_3(16))
	assert(!isPowerOfTwo_1(3))
	assert(!isPowerOfTwo_2(3))
	assert(!isPowerOfTwo_3(3))
	assert(isPowerOfTwo_1(4))
	assert(isPowerOfTwo_2(4))
	assert(isPowerOfTwo_3(4))
	assert(!isPowerOfTwo_1(5))
	assert(!isPowerOfTwo_2(5))
	assert(!isPowerOfTwo_3(5))
}
