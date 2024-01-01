/*
 * @Date: 2023-03-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-01
 * @FilePath: /algorithm/golang/1599_min_operations_max_profit/min_operations_max_profit.go
 */

package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minOperationsMaxProfit(customers []int, boardingCost, runningCost int) int {
	ans := -1
	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}
	var maxProfit, totalProfit, operations, customersCount int
	for _, c := range customers {
		operations++
		customersCount += c
		curCustomers := min(customersCount, 4)
		customersCount -= curCustomers
		totalProfit += boardingCost*curCustomers - runningCost
		if totalProfit > maxProfit {
			maxProfit = totalProfit
			ans = operations
		}
	}
	if customersCount == 0 {
		return ans
	}
	profitEachTime := boardingCost*4 - runningCost
	if profitEachTime <= 0 {
		return ans
	}
	if customersCount > 0 {
		fullTimes := customersCount / 4
		totalProfit += profitEachTime * fullTimes
		operations += fullTimes
		if totalProfit > maxProfit {
			maxProfit = totalProfit
			ans = operations
		}
		remainingCustomers := customersCount % 4
		remainingProfit := boardingCost*remainingCustomers - runningCost
		totalProfit += remainingProfit
		if totalProfit > maxProfit {
			maxProfit = totalProfit
			operations++
			ans++
		}
	}
	return ans
}

func main() {
	tests := []struct {
		customers    []int
		boardingCost int
		runningCost  int
		ans          int
	}{
		{[]int{8, 3}, 5, 6, 3},
		{[]int{10, 9, 6}, 6, 4, 7},
		{[]int{3, 4, 0, 5, 1}, 1, 92, -1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minOperationsMaxProfit(test.customers, test.boardingCost, test.runningCost), index)
	}
}
