/*
 * @Date: 2022-12-01
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-01
 * @FilePath: /algorithm/1779_nearest_valid_point/nearest_valid_point.go
 */

package main

import "math"

func nearestValidPoint(x, y int, points [][]int) int {
	abs := func(x int) int {
		if x < 0 {
			return -x
		}
		return x
	}

	ans := -1
	minDist := math.MaxInt32
	for i, p := range points {
		if p[0] == x || p[1] == y {
			dist := abs(p[0]-x) + abs(p[1]-y)
			if dist < minDist {
				minDist = dist
				ans = i
			}
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
		x := 3
		y := 4
		points := [][]int{{1, 2}, {3, 1}, {2, 4}, {2, 3}, {4, 4}}
		ans := 2
		assert(nearestValidPoint(x, y, points) == ans)
	}

	{
		x := 3
		y := 4
		points := [][]int{{3, 4}}
		ans := 0
		assert(nearestValidPoint(x, y, points) == ans)
	}
}
