/*
 * @Date: 2022-06-25
 * @LastEditors: Mengsen Wang mengsen_wang@163.com
 * @LastEditTime: 2022-06-25
 * @FilePath: /algorithm/091_min_cost/min_cost.go
 */

package main

func minCost(costs [][]int) int {
	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}
	dp := costs[0]
	for _, cost := range costs[1:] {
		dpNew := make([]int, 3)
		for j, c := range cost {
			dpNew[j] = min(dp[(j+1)%3], dp[(j+2)%3]) + c
		}
		dp = dpNew
	}
	return min(min(dp[0], dp[1]), dp[2])
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(minCost([][]int{{17, 2, 17}, {16, 16, 5}, {14, 3, 19}}) == 10)
	assert(minCost([][]int{{7, 6, 2}}) == 2)
}
