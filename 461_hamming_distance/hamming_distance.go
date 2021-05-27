/*
 * @Date: 2021-05-27 09:41:14
 * @Author: mengsenwang
 * @LastEditors: mengsenwang
 * @LastEditTime: 2021-05-27 09:52:36
 */

package main

import "math/bits"

func hammingDistance(x, y int) int {
	return bits.OnesCount(uint(x ^ y))
}

func hammingDistance_move_bits(x, y int) (ans int) {
	for s := x ^ y; s > 0; s >>= 1 {
		ans += s & 1
	}
	return
}

func hammingDistance_BK(x, y int) (ans int) {
	for s := x ^ y; s > 0; s &= s - 1 {
		ans++
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed!")
		}
	}
	assert(hammingDistance(1, 4) == 2)
	assert(hammingDistance_move_bits(1, 4) == 2)
	assert(hammingDistance_BK(1, 4) == 2)
}
