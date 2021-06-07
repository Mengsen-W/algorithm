/*
 * @Date: 2021-06-07 08:26:29
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-07 08:36:06
 */

package main

func findTargetSumWays(nums []int, target int) int {
	sum := 0
	for _, v := range nums {
		sum += v
	}
	diff := sum - target
	if diff < 0 || diff%2 == 1 {
		return 0
	}
	neg := diff / 2
	dp := make([]int, neg+1)
	dp[0] = 1
	for _, num := range nums {
		for j := neg; j >= num; j-- {
			dp[j] += dp[j-num]
		}
	}
	return dp[neg]
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed!")
		}
	}
	{
		nums := []int{1, 1, 1, 1, 1}
		target := 3
		ans := 5
		assert(findTargetSumWays(nums, target) == ans)
	}
	{
		nums := []int{1}
		target := 1
		ans := 1
		assert(findTargetSumWays(nums, target) == ans)
	}
}
