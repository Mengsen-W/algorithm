/*
 * @Date: 2021-10-25 00:50:14
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-25 01:17:53
 */

package main

func searchMatrix(matrix [][]int, target int) bool {
	m, n := len(matrix), len(matrix[0])
	x, y := 0, n-1
	for x < m && y >= 0 {
		if matrix[x][y] == target {
			return true
		} else if matrix[x][y] < target {
			x += 1
		} else if matrix[x][y] > target {
			y -= 1
		}
	}
	return false
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		matrix := [][]int{{1, 4, 7, 11, 15},
			{2, 5, 8, 12, 19},
			{3, 6, 9, 16, 22},
			{10, 13, 14, 17, 24},
			{18, 21, 23, 26, 30}}
		target := 5
		assert(searchMatrix(matrix, target))
	}
	{
		matrix := [][]int{{1, 4, 7, 11, 15},
			{2, 5, 8, 12, 19},
			{3, 6, 9, 16, 22},
			{10, 13, 14, 17, 24},
			{18, 21, 23, 26, 30}}
		target := 20
		assert(!searchMatrix(matrix, target))
	}
	{
		matrix := [][]int{{-5}}
		target := -10
		assert(!searchMatrix(matrix, target))
	}
}
