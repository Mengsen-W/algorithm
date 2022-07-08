/*
 * @Date: 2022-07-08
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-08
 * @FilePath: /algorithm/1217_min_cost_to_move_chips/min_cost_to_move_chips.go
 */

package main

func minCostToMoveChips(position []int) int {
	cnt := [2]int{}
	for _, p := range position {
		cnt[p%2]++
	}
	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}
	return min(cnt[0], cnt[1])
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(minCostToMoveChips([]int{1, 2, 3}) == 1)
	assert(minCostToMoveChips([]int{2, 2, 2, 3, 3}) == 2)
	assert(minCostToMoveChips([]int{1, 10000000}) == 1)
}
