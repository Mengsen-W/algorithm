/*
 * @Date: 2022-12-04
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-04
 * @FilePath: /algorithm/1774_closest_cost/closest_cost.go
 */

package main

func closestCost(baseCosts []int, toppingCosts []int, target int) int {
	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}
	x := baseCosts[0]
	for _, c := range baseCosts {
		x = min(x, c)
	}
	if x > target {
		return x
	}
	can := make([]bool, target+1)
	ans := 2*target - x
	for _, c := range baseCosts {
		if c <= target {
			can[c] = true
		} else {
			ans = min(ans, c)
		}
	}
	for _, c := range toppingCosts {
		for count := 0; count < 2; count++ {
			for i := target; i > 0; i-- {
				if can[i] && i+c > target {
					ans = min(ans, i+c)
				}
				if i-c > 0 {
					can[i] = can[i] || can[i-c]
				}
			}
		}
	}
	for i := 0; i <= ans-target; i++ {
		if can[target-i] {
			return target - i
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
		baseCosts := []int{1, 7}
		toppingCosts := []int{3, 4}
		target := 10
		ans := 10
		assert(closestCost(baseCosts, toppingCosts, target) == ans)
	}

	{
		baseCosts := []int{2, 3}
		toppingCosts := []int{4, 5, 100}
		target := 18
		ans := 17
		assert(closestCost(baseCosts, toppingCosts, target) == ans)
	}

	{
		baseCosts := []int{3, 10}
		toppingCosts := []int{2, 5}
		target := 9
		ans := 8
		assert(closestCost(baseCosts, toppingCosts, target) == ans)
	}

	{
		baseCosts := []int{10}
		toppingCosts := []int{1}
		target := 1
		ans := 10
		assert(closestCost(baseCosts, toppingCosts, target) == ans)
	}
}
