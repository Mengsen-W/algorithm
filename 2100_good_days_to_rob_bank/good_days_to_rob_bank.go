/*
 * @Date: 2022-03-06 00:58:08
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-06 01:24:01
 * @FilePath: /algorithm/2100_good_days_to_rob_bank/good_days_to_rob_bank.go
 */

package main

import "reflect"

func goodDaysToRobBank(security []int, time int) (ans []int) {
	n := len(security)
	left := make([]int, n)
	right := make([]int, n)
	for i := 1; i < n; i++ {
		if security[i] <= security[i-1] {
			left[i] = left[i-1] + 1
		}
		if security[n-i-1] <= security[n-i] {
			right[n-i-1] = right[n-i] + 1
		}
	}

	for i := time; i < n-time; i++ {
		if left[i] >= time && right[i] >= time {
			ans = append(ans, i)
		}
	}
	return
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("not equal")
		}
	}

	assert(goodDaysToRobBank([]int{5, 3, 3, 3, 5, 6, 2}, 2), []int{2, 3})
	assert(goodDaysToRobBank([]int{1, 1, 1, 1, 1}, 0), []int{0, 1, 2, 3, 4})
	assert(goodDaysToRobBank([]int{1, 2, 3, 4, 5, 6}, 2), nil)
	assert(goodDaysToRobBank([]int{1}, 5), nil)
}
