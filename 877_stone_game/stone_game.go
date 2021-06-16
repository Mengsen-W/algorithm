/*
 * @Date: 2021-06-16 09:10:01
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-16 09:14:26
 */

package main

func stoneGame(piles []int) bool {
	max := func(x, y int) int {
		if x > y {
			return x
		}
		return y
	}
	length := len(piles)
	dp := make([]int, length)
	for i := 0; i < length; i++ {
		dp[i] = piles[i]
	}
	for i := length - 2; i >= 0; i-- {
		for j := i + 1; j < length; j++ {
			dp[j] = max(piles[i]-dp[j], piles[j]-dp[j-1])
		}
	}
	return dp[length-1] > 0
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed!")
		}
	}
	assert(stoneGame([]int{5, 3, 4, 5}))
}
