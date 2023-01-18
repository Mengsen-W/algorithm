/*
 * @Date: 2021-08-16 21:17:11
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-16 21:20:12
 */

package main

import (
	"math/bits"
)

func countArrangement(n int) int {
	f := make([]int, 1<<n)
	f[0] = 1
	for mask := 1; mask < 1<<n; mask++ {
		num := bits.OnesCount(uint(mask))
		for i := 0; i < n; i++ {
			if mask>>i&1 > 0 && (num%(i+1) == 0 || (i+1)%num == 0) {
				f[mask] += f[mask^1<<i]
			}
		}
	}
	return f[1<<n-1]
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(countArrangement(2) == 2)
}
