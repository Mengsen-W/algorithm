/*
 * @Date: 2021-05-13 08:33:01
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-13 08:47:16
 */

package main

func numWays(steps, arrLen int) int {
	const mod = 1e9 + 7
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}
	maxColumn := min(arrLen-1, steps)
	dp := make([]int, maxColumn+1)
	dp[0] = 1
	for i := 1; i <= steps; i++ {
		dpNext := make([]int, maxColumn+1)
		for j := 0; j <= maxColumn; j++ {
			dpNext[j] = dp[j]
			if j-1 >= 0 {
				dpNext[j] = (dpNext[j] + dp[j-1]) % mod
			}
			if j+1 <= maxColumn {
				dpNext[j] = (dpNext[j] + dp[j+1]) % mod
			}
		}
		dp = dpNext
	}
	return dp[0]
}

func main() {
	if numWays(3, 2) != 4 {
		panic("Not Passed!")
	}
	if numWays(2, 4) != 2 {
		panic("Not Passed!")
	}
}
