/*
 * @Date: 2021-06-01 09:00:55
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-01 09:14:02
 */

package main

import "fmt"

func canEat(candiesCount []int, queries [][]int) []bool {
	n := len(candiesCount)

	// 前缀和
	sum := make([]int, n)
	sum[0] = candiesCount[0]
	for i := 1; i < n; i++ {
		sum[i] = sum[i-1] + candiesCount[i]
	}

	ans := make([]bool, len(queries))
	for i, q := range queries {
		favoriteType, favoriteDay, dailyCap := q[0], q[1], q[2]

		x1 := favoriteDay + 1
		y1 := (favoriteDay + 1) * dailyCap
		x2 := 1
		if favoriteType > 0 {
			x2 = sum[favoriteType-1] + 1
		}
		y2 := sum[favoriteType]

		ans[i] = !(x1 > y2 || y1 < x2)
	}
	return ans
}

func main() {
	assert := func(b []bool) {
		for i, s := range b {
			fmt.Println(i, s)
		}
	}
	{
		candiesCount := []int{7, 4, 5, 3, 8}
		queries := [][]int{{0, 2, 2}, {4, 2, 4}, {2, 13, 1000000000}}
		assert(canEat(candiesCount, queries))
	}
	{
		candiesCount := []int{5, 2, 6, 4, 1}
		queries := [][]int{{3, 1, 2}, {4, 10, 3}, {3, 10, 100}, {4, 100, 30}, {1, 3, 1}}
		assert(canEat(candiesCount, queries))
	}
}
