/*
 * @Date: 2022-04-05 10:25:35
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-05 10:42:12
 * @FilePath: /algorithm/762_count_prime_set_bits/count_prime_set_bits.go
 */

package main

import "math/bits"

func countPrimeSetBits(left, right int) (ans int) {
	for x := left; x <= right; x++ {
		if 1<<bits.OnesCount(uint(x))&665772 != 0 {
			ans++
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
	assert(countPrimeSetBits(6, 10) == 4)
	assert(countPrimeSetBits(10, 15) == 5)
}
