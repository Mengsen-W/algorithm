/*
 * @Date: 2022-07-12
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-12
 * @FilePath: /algorithm/1252_odd_cells/odd_cells.go
 */

package main

func oddCells(m, n int, indices [][]int) int {
	rows := make([]int, m)
	cols := make([]int, n)
	for _, p := range indices {
		rows[p[0]]++
		cols[p[1]]++
	}
	oddx := 0
	for _, row := range rows {
		oddx += row % 2
	}
	oddy := 0
	for _, col := range cols {
		oddy += col % 2
	}
	return oddx*(n-oddy) + (m-oddx)*oddy
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(oddCells(2, 3, [][]int{{0, 1}, {1, 1}}) == 6)
	assert(oddCells(2, 2, [][]int{{1, 1}, {0, 0}}) == 0)
}
