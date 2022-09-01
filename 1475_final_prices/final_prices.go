/*
 * @Date: 2022-09-01
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-01
 * @FilePath: /algorithm/1475_final_prices/final_prices.go
 */

package main

import "reflect"

func finalPrices(prices []int) []int {
	n := len(prices)
	ans := make([]int, n)
	st := []int{0}
	for i := n - 1; i >= 0; i-- {
		p := prices[i]
		for len(st) > 1 && st[len(st)-1] > p {
			st = st[:len(st)-1]
		}
		ans[i] = p - st[len(st)-1]
		st = append(st, p)
	}
	return ans
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	{
		prices := []int{8, 4, 6, 2, 3}
		ans := []int{4, 2, 4, 2, 3}
		assert(finalPrices(prices), ans)
	}

	{
		prices := []int{1, 2, 3, 4, 5}
		ans := []int{1, 2, 3, 4, 5}
		assert(finalPrices(prices), ans)
	}

	{
		prices := []int{10, 1, 1, 6}
		ans := []int{9, 0, 1, 6}
		assert(finalPrices(prices), ans)
	}
}
