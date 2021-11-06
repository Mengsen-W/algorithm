/*
 * @Date: 2021-11-06 00:36:09
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-06 00:48:44
 */

package main

func longestSubsequence(arr []int, difference int) (ans int) {
	dp := map[int]int{}
	for _, v := range arr {
		dp[v] = dp[v-difference] + 1
		if dp[v] > ans {
			ans = dp[v]
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

	{
		arr := []int{1, 2, 3, 4}
		difference := 1
		assert(longestSubsequence(arr, difference) == 4)
	}
	{
		arr := []int{1, 3, 5, 7}
		difference := 1
		assert(longestSubsequence(arr, difference) == 1)
	}
	{
		arr := []int{1, 5, 7, 8, 5, 3, 4, 2, 1}
		difference := -2
		assert(longestSubsequence(arr, difference) == 4)
	}
}
