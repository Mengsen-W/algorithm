/*
 * @Date: 2021-06-09 08:34:16
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-09 08:46:30
 */

package main

func profitableSchemes(n, minProfit int, group, profit []int) (sum int) {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	const mod int = 1e9 + 7
	dp := make([][]int, n+1)
	for i := range dp {
		dp[i] = make([]int, minProfit+1)
		dp[i][0] = 1
	}
	for i, members := range group {
		earn := profit[i]
		for j := n; j >= members; j-- {
			for k := minProfit; k >= 0; k-- {
				dp[j][k] = (dp[j][k] + dp[j-members][max(0, k-earn)]) % mod
			}
		}
	}
	return dp[n][minProfit]
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed!")
		}
	}

	{
		n := 5
		minProfit := 3
		group := []int{2, 2}
		profit := []int{2, 3}
		ans := 2
		assert(profitableSchemes(n, minProfit, group, profit) == ans)
	}
	{
		n := 10
		minProfit := 5
		group := []int{2, 3, 5}
		profit := []int{6, 7, 8}
		ans := 7
		assert(profitableSchemes(n, minProfit, group, profit) == ans)
	}
}
