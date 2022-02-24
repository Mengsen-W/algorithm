/*
 * @Date: 2022-02-23 23:51:52
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-24 00:18:06
 * @FilePath: /algorithm/1706_find_ball/find_ball.go
 */

package main

import "reflect"

func findBall(grid [][]int) []int {
	n := len(grid[0])
	ans := make([]int, n)
	for j := range ans {
		col := j // 球的初始列
		for _, row := range grid {
			dir := row[col]
			col += dir                                  // 移动球
			if col < 0 || col == n || row[col] != dir { // 到达侧边或 V 形
				col = -1
				break
			}
		}
		ans[j] = col // col >= 0 为成功到达底部
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
		grid := [][]int{{1, 1, 1, -1, -1}, {1, 1, 1, -1, -1}, {-1, -1, -1, 1, 1}, {1, 1, 1, 1, -1}, {-1, -1, -1, -1, -1}}
		ans := []int{1, -1, -1, -1, -1}
		assert(findBall(grid), ans)
	}

	{
		grid := [][]int{{-1}}
		ans := []int{-1}
		assert(findBall(grid), ans)
	}

	{
		grid := [][]int{{1, 1, 1, 1, 1, 1}, {-1, -1, -1, -1, -1, -1}, {1, 1, 1, 1, 1, 1}, {-1, -1, -1, -1, -1, -1}}
		ans := []int{0, 1, 2, 3, 4, -1}
		assert(findBall(grid), ans)
	}
}
