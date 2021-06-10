/*
 * @Date: 2021-06-10 09:11:57
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-10 09:17:38
 */

package main

func change(amount int, coins []int) int {
	dp := make([]int, amount+1)
	dp[0] = 1
	for _, coin := range coins {
		for i := coin; i <= amount; i++ {
			dp[i] += dp[i-coin]
		}
	}
	return dp[amount]

}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed!")
		}
	}
	{
		amount := 5
		coins := []int{1, 2, 5}
		ans := 4
		assert(change(amount, coins) == ans)
	}
	{
		amount := 3
		coins := []int{2}
		ans := 0
		assert(change(amount, coins) == ans)
	}
	{
		amount := 10
		coins := []int{10}
		ans := 1
		assert(change(amount, coins) == ans)
	}
}
