/*
 * @Date: 2021-06-08 08:38:58
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-08 08:51:56
 * @FilePath: \algorithm\1049_last_stone_weight_ii\last_stone_weight_ii.go
 * @Description: file content
 */
package main

func lastStoneWeightII(stones []int) int {
	sum := 0
	for _, v := range stones {
		sum += v
	}
	m := sum / 2
	dp := make([]bool, m+1)
	dp[0] = true
	for _, weight := range stones {
		for j := m; j >= weight; j-- {
			dp[j] = dp[j] || dp[j-weight]
		}
	}
	for j := m; ; j-- {
		if dp[j] {
			return sum - 2*j
		}
	}
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed!")
		}
	}

	{
		stones := []int{2, 7, 4, 1, 8, 1}
		assert(lastStoneWeightII(stones) == 1)
	}
	{
		stones := []int{31, 26, 33, 21, 40}
		assert(lastStoneWeightII(stones) == 5)
	}
	{
		stones := []int{1, 2}
		assert(lastStoneWeightII(stones) == 1)
	}
}
