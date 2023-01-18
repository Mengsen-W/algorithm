/*
 * @Date: 2022-12-10
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-10
 * @FilePath: /algorithm/1691_max_height/max_height.go
 */

package main

import "sort"

func maxHeight(cuboids [][]int) int {
	check := func(a, b []int) bool {
		return a[0] <= b[0] && a[1] <= b[1] && a[2] <= b[2]
	}

	max := func(a, b int) int {
		if a >= b {
			return a
		} else {
			return b
		}
	}

	for _, v := range cuboids {
		sort.Ints(v)
	}
	sort.Slice(cuboids, func(i, j int) bool {
		return cuboids[i][0]+cuboids[i][1]+cuboids[i][2] < cuboids[j][0]+cuboids[j][1]+cuboids[j][2]
	})

	memo := []int{}
	for i := 0; i < len(cuboids); i++ {
		memo = append(memo, -1)
	}

	var dfs func(top, index int) int
	dfs = func(top, index int) int {
		if index == len(cuboids) {
			return 0
		}
		if top != -1 && memo[top] != -1 {
			return memo[top]
		}
		height := dfs(top, index+1)
		if top == -1 || check(cuboids[top], cuboids[index]) {
			height = max(height, cuboids[index][2]+dfs(index, index+1))
		}
		if top != -1 {
			memo[top] = height
		}
		return height
	}
	return dfs(-1, 0)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		cuboids := [][]int{{50, 45, 20}, {95, 37, 53}, {45, 23, 12}}
		assert(maxHeight(cuboids) == 190)
	}

	{
		cuboids := [][]int{{38, 25, 45}, {76, 35, 3}}
		assert(maxHeight(cuboids) == 76)
	}

	{
		cuboids := [][]int{{7, 11, 17}, {7, 17, 11}, {11, 7, 17}, {11, 17, 7}, {17, 7, 11}, {17, 11, 7}}
		assert(maxHeight(cuboids) == 102)
	}
}
