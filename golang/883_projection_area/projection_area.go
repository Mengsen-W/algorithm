/*
 * @Date: 2022-04-26 09:35:45
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-26 09:44:03
 * @FilePath: /algorithm/883_projection_area/projection_area.go
 */

package main

func projectionArea(grid [][]int) int {
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}
	var xyArea, yzArea, zxArea int
	for i, row := range grid {
		yzHeight, zxHeight := 0, 0
		for j, v := range row {
			if v > 0 {
				xyArea++
			}
			yzHeight = max(yzHeight, v)
			zxHeight = max(zxHeight, grid[j][i])
		}
		yzArea += yzHeight
		zxArea += zxHeight
	}
	return xyArea + yzArea + zxArea
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(projectionArea([][]int{{1, 2}, {3, 4}}) == 17)
	assert(projectionArea([][]int{{2}}) == 5)
	assert(projectionArea([][]int{{1, 0}, {0, 2}}) == 8)
}
