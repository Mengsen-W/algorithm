/*
 * @Date: 2021-06-12 09:53:31
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-12 10:10:49
 */

package main

import "math"

func largestNumber(cost []int, target int) string {
	max := func(a, b int) int {
		if a < b {
			return b
		}
		return a
	}
	dp := make([]int, target+1)
	for i := range dp {
		dp[i] = math.MinInt32
	}
	dp[0] = 0
	for _, c := range cost {
		for j := c; j <= target; j++ {
			dp[j] = max(dp[j], dp[j-c]+1)
		}
	}
	if dp[target] < 0 {
		return "0"
	}
	ans := make([]byte, 0, dp[target])
	for i, j := 8, target; i >= 0; i-- {
		for c := cost[i]; j >= c && dp[j] == dp[j-c]+1; j -= c {
			ans = append(ans, byte('1'+i))
		}
	}
	return string(ans)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed!")
		}
	}
	{
		cost := []int{4, 3, 2, 5, 6, 7, 2, 5, 5}
		target := 9
		res := "7772"
		assert(largestNumber(cost, target) == res)
	}
	{
		cost := []int{7, 6, 5, 5, 5, 6, 8, 7, 8}
		target := 12
		res := "85"
		assert(largestNumber(cost, target) == res)
	}
	{
		cost := []int{2, 4, 6, 2, 4, 6, 4, 4, 4}
		target := 5
		res := "0"
		assert(largestNumber(cost, target) == res)
	}
	{
		cost := []int{6, 10, 15, 40, 40, 40, 40, 40, 40}
		target := 47
		res := "32211"
		assert(largestNumber(cost, target) == res)
	}
}
