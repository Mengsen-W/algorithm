/*
 * @Date: 2021-06-10 09:11:57
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-25
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

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
	tests := []struct {
		amount int
		coins  []int
		ans    int
	}{
		{5, []int{1, 2, 5}, 4},
		{3, []int{2}, 0},
		{10, []int{10}, 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, change(test.amount, test.coins), index)
	}
}
