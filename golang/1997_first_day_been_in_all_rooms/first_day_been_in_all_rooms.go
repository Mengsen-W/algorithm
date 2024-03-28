/*
 * @Date: 2024-03-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-28
 * @FilePath: /algorithm/golang/1997_first_day_been_in_all_rooms/first_day_been_in_all_rooms.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func firstDayBeenInAllRooms(nextVisit []int) int {
	mod := 1000000007
	dp := make([]int, len(nextVisit))

	dp[0] = 2 // 初始化原地待一天 + 访问下一个房间一天
	for i := 1; i < len(nextVisit); i++ {
		to := nextVisit[i]
		dp[i] = dp[i-1] + 2
		if to != 0 {
			dp[i] = (dp[i] - dp[to-1] + mod) % mod // 避免负数
		}
		dp[i] = (dp[i] + dp[i-1]) % mod // 题目保证n >= 2
	}

	return dp[len(nextVisit)-2]
}

func main() {
	tests := []struct {
		nextVisit []int
		ans       int
	}{
		{[]int{0, 0}, 2},
		{[]int{0, 0, 2}, 6},
		{[]int{0, 1, 2, 0}, 6},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, firstDayBeenInAllRooms(test.nextVisit), index)
	}
}
