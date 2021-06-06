/*
 * @Date: 2021-06-06 09:36:34
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-06 09:51:36
 */

package main

import "strings"

func findMaxForm(strs []string, m, n int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	dp := make([][]int, m+1)
	for i := range dp {
		dp[i] = make([]int, n+1)
	}
	for _, s := range strs {
		zeros := strings.Count(s, "0")
		ones := len(s) - zeros
		for j := m; j >= zeros; j-- {
			for k := n; k >= ones; k-- {
				dp[j][k] = max(dp[j][k], dp[j-zeros][k-ones]+1)
			}
		}
	}
	return dp[m][n]
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed!")
		}
	}
	{
		strs := []string{"10", "0001", "111001", "1", "0"}
		m := 5
		n := 3
		assert(findMaxForm(strs, m, n) == 4)
	}
	{
		strs := []string{"10", "0", "1"}
		m := 1
		n := 1
		assert(findMaxForm(strs, m, n) == 2)
	}
}
