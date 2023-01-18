/*
 * @Date: 2021-06-23 08:45:09
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-23 09:01:50
 */

package main

import "math/bits"

func hammingWeight(num uint32) int {
	return bits.OnesCount(uint(num))
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed!")
		}
	}
	assert(hammingWeight(0b00000000000000000000000000001011) == 3)
	assert(hammingWeight(0b00000000000000000000000010000000) == 1)
	assert(hammingWeight(0b11111111111111111111111111111101) == 31)
}
