/*
 * @Date: 2023-03-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-05
 * @FilePath: /algorithm/golang/1599_min_operations_max_profit/min_operations_max_profit.go
 */

package main

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
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		customers := []int{8, 3}
		boardingCost := 5
		runningCost := 6
		ans := 3
		assert(minOperationsMaxProfit(customers, boardingCost, runningCost) == ans)
	}

	{
		customers := []int{10, 9, 6}
		boardingCost := 6
		runningCost := 4
		ans := 7
		assert(minOperationsMaxProfit(customers, boardingCost, runningCost) == ans)
	}

	{
		customers := []int{3, 4, 0, 5, 1}
		boardingCost := 1
		runningCost := 92
		ans := -1
		assert(minOperationsMaxProfit(customers, boardingCost, runningCost) == ans)
	}
}
