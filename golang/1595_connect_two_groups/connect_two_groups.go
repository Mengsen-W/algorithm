/*
 * @Date: 2023-06-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-20
 * @FilePath: /algorithm/golang/1595_connect_two_groups/connect_two_groups.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify"
)

func connectTwoGroups(cost [][]int) int {
	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}
	size1, size2, m := len(cost), len(cost[0]), 1<<len(cost[0])
	dp1, dp2 := make([]int, m), make([]int, m)
	for s := 1; s < m; s++ {
		dp1[s] = 0x3f3f3f3f
	}
	for i := 1; i <= size1; i++ {
		for s := 0; s < m; s++ {
			dp2[s] = 0x3f3f3f3f
			for k := 0; k < size2; k++ {
				if (s & (1 << k)) == 0 {
					continue
				}
				dp2[s] = min(dp2[s], dp2[s^(1<<k)]+cost[i-1][k])
				dp2[s] = min(dp2[s], dp1[s]+cost[i-1][k])
				dp2[s] = min(dp2[s], dp1[s^(1<<k)]+cost[i-1][k])
			}
		}
		dp1, dp2 = dp2, dp1
	}
	return dp1[m-1]
}

func main() {
	{
		cost := [][]int{{15, 96}, {36, 2}}
		ans := 17
		assert.Equal(&testing.B{}, connectTwoGroups(cost), ans)
	}

	{
		cost := [][]int{{1, 3, 5}, {4, 1, 1}, {1, 5, 3}}
		ans := 4
		assert.Equal(&testing.B{}, connectTwoGroups(cost), ans)
	}

	{
		cost := [][]int{{2, 5, 1}, {3, 4, 7}, {8, 1, 2}, {6, 2, 4}, {3, 8, 8}}
		ans := 10
		assert.Equal(&testing.B{}, connectTwoGroups(cost), ans)
	}
}
