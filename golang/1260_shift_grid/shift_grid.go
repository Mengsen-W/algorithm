/*
 * @Date: 2022-07-20
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-07-20
 * @FilePath: /algorithm/1260_shift_grid/shift_grid.go
 */

package main

import "reflect"

func shiftGrid(grid [][]int, k int) [][]int {
	m, n := len(grid), len(grid[0])
	ans := make([][]int, m)
	for i := range ans {
		ans[i] = make([]int, n)
	}
	for i, row := range grid {
		for j, v := range row {
			index1 := (i*n + j + k) % (m * n)
			ans[index1/n][index1%n] = v
		}
	}
	return ans
}

func main() {
	assert := func(a, b [][]int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	{
		grid := [][]int{{1, 2, 3}, {4, 5, 6}, {7, 8, 9}}
		k := 1
		ans := [][]int{{9, 1, 2}, {3, 4, 5}, {6, 7, 8}}
		assert(shiftGrid(grid, k), ans)
	}

	{
		grid := [][]int{{3, 8, 1, 9}, {19, 7, 2, 5}, {4, 6, 11, 10}, {12, 0, 21, 13}}
		k := 4
		ans := [][]int{{12, 0, 21, 13}, {3, 8, 1, 9}, {19, 7, 2, 5}, {4, 6, 11, 10}}
		assert(shiftGrid(grid, k), ans)
	}

	{
		grid := [][]int{{1, 2, 3}, {4, 5, 6}, {7, 8, 9}}
		k := 9
		ans := [][]int{{1, 2, 3}, {4, 5, 6}, {7, 8, 9}}
		assert(shiftGrid(grid, k), ans)
	}
}
