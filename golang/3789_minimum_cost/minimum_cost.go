// Package main ...
package main

import "fmt"

func minimumCost(cost1 int, cost2 int, costBoth int, need1 int, need2 int) int64 {
	var totalCost int64 = 0

	// 分别购买更便宜，不需要物品 3
	if cost1+cost2 <= costBoth {
		totalCost = int64(cost1)*int64(need1) + int64(cost2)*int64(need2)
	} else {
		// 优先购买物品 3，直到一种需求满足
		cnt3 := min(need1, need2)
		totalCost = int64(cnt3) * int64(costBoth)

		need1 -= cnt3
		need2 -= cnt3

		// 类型 1 的需求还未被满足，选择物品 1 或物品 3 中更便宜的
		if need1 > 0 {
			totalCost += int64(need1) * int64(min(cost1, costBoth))
		} else if need2 > 0 {
			// 类型 2 的需求还未被满足，选择物品 2 或物品 3 中更便宜的
			totalCost += int64(need2) * int64(min(cost2, costBoth))
		}
	}

	return totalCost
}

func main() {
	tests := []struct {
		cost1    int
		cost2    int
		costBoth int
		need1    int
		need2    int
		ans      int64
	}{
		{3, 2, 1, 3, 2, 3},
		{5, 4, 15, 2, 3, 22},
		{5, 4, 15, 0, 0, 0},
	}

	for index, test := range tests {
		if minimumCost(test.cost1, test.cost2, test.costBoth, test.need1, test.need2) != test.ans {
			panic(fmt.Sprintf("test %d failed", index))
		}
	}
}
