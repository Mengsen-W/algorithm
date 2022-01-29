/*
 * @Date: 2022-01-29 00:52:25
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-29 01:02:33
 */

package main

import "reflect"

type pair struct{ x, y int }

var dirs = []pair{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}

func highestPeak(isWater [][]int) [][]int {
	m, n := len(isWater), len(isWater[0])
	ans := make([][]int, m)
	for i := range ans {
		ans[i] = make([]int, n)
		for j := range ans[i] {
			ans[i][j] = -1 // -1 表示该格子尚未被访问过
		}
	}
	q := []pair{}
	for i, row := range isWater {
		for j, water := range row {
			if water == 1 { // 将所有水域入队
				ans[i][j] = 0
				q = append(q, pair{i, j})
			}
		}
	}
	for len(q) > 0 {
		p := q[0]
		q = q[1:]
		for _, d := range dirs {
			if x, y := p.x+d.x, p.y+d.y; 0 <= x && x < m && 0 <= y && y < n && ans[x][y] == -1 {
				ans[x][y] = ans[p.x][p.y] + 1
				q = append(q, pair{x, y})
			}
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

	assert(highestPeak([][]int{{0, 1}, {0, 0}}),
		[][]int{{1, 0}, {2, 1}})
	assert(highestPeak([][]int{{0, 0, 1}, {1, 0, 0}, {0, 0, 0}}),
		[][]int{{1, 1, 0}, {0, 1, 1}, {1, 2, 2}})
}
