/*
 * @Date: 2021-11-12 01:08:02
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-12 01:17:13
 */
package main

import "math"

func getMoneyAmount(n int) int {
	f := make([][]int, n+1)
	for i := range f {
		f[i] = make([]int, n+1)
	}
	for i := n - 1; i >= 1; i-- {
		for j := i + 1; j <= n; j++ {
			minCost := math.MaxInt32
			for k := i; k < j; k++ {
				cost := k + max(f[i][k-1], f[k+1][j])
				if cost < minCost {
					minCost = cost
				}
			}
			f[i][j] = minCost
		}
	}
	return f[1][n]
}

func max(a, b int) int {
	if b > a {
		return b
	}
	return a
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(getMoneyAmount(1) == 0)
	assert(getMoneyAmount(2) == 1)
	assert(getMoneyAmount(10) == 16)
}
