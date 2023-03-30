/*
 * @Date: 2023-03-30
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-30
 * @FilePath: /algorithm/golang/1637_max_width_of_vertical_area/max_width_of_vertical_area.go
 */

// Package main ..
package main

import "sort"

func maxWidthOfVerticalArea(points [][]int) (ans int) {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	sort.Slice(points, func(i, j int) bool {
		return points[i][0] < points[j][0]
	})
	for i := 1; i < len(points); i++ {
		ans = max(ans, points[i][0]-points[i-1][0])
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		points := [][]int{{8, 7}, {9, 9}, {7, 4}, {9, 7}}
		ans := 1
		assert(maxWidthOfVerticalArea(points) == ans)
	}

	{
		points := [][]int{{3, 1}, {9, 0}, {1, 0}, {1, 4}, {5, 3}, {8, 8}}
		ans := 3
		assert(maxWidthOfVerticalArea(points) == ans)
	}
}
