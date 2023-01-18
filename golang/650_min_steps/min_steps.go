/*
 * @Date: 2021-09-19 09:01:11
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-19 09:16:53
 */

package main

import (
	"math"
)

func minSteps(n int) int {
	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}
	f := make([]int, n+1)
	for i := 2; i <= n; i++ {
		f[i] = math.MaxInt32
		for j := 1; j*j <= i; j++ {
			if i%j == 0 {
				f[i] = min(f[i], f[j]+i/j)
				f[i] = min(f[i], f[i/j]+j)
			}
		}
	}
	return f[n]
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(minSteps(3) == 3)
	assert(minSteps(1) == 0)
}
