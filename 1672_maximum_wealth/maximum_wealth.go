/*
 * @Date: 2022-04-14 09:16:17
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-14 09:22:20
 * @FilePath: /algorithm/1672_maximum_wealth/maximum_wealth.go
 */

package main

func maximumWealth(accounts [][]int) (ans int) {
	for _, account := range accounts {
		sum := 0
		for _, val := range account {
			sum += val
		}
		if sum > ans {
			ans = sum
		}
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(maximumWealth([][]int{{1, 2, 3}, {3, 2, 1}}) == 6)
	assert(maximumWealth([][]int{{1, 5}, {7, 3}, {3, 5}}) == 10)
	assert(maximumWealth([][]int{{2, 8, 7}, {7, 1, 3}, {1, 9, 5}}) == 17)
}
