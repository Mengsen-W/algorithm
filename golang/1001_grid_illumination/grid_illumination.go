/*
 * @Date: 2022-02-08 00:28:39
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-08 00:42:52
 */

package main

import "reflect"

func gridIllumination(n int, lamps, queries [][]int) []int {
	type pair struct{ x, y int }
	points := map[pair]bool{}
	row := map[int]int{}
	col := map[int]int{}
	diagonal := map[int]int{}
	antiDiagonal := map[int]int{}
	for _, lamp := range lamps {
		r, c := lamp[0], lamp[1]
		p := pair{r, c}
		if points[p] {
			continue
		}
		points[p] = true
		row[r]++
		col[c]++
		diagonal[r-c]++
		antiDiagonal[r+c]++
	}

	ans := make([]int, len(queries))
	for i, query := range queries {
		r, c := query[0], query[1]
		if row[r] > 0 || col[c] > 0 || diagonal[r-c] > 0 || antiDiagonal[r+c] > 0 {
			ans[i] = 1
		}
		for x := r - 1; x <= r+1; x++ {
			for y := c - 1; y <= c+1; y++ {
				if x < 0 || y < 0 || x >= n || y >= n || !points[pair{x, y}] {
					continue
				}
				delete(points, pair{x, y})
				row[x]--
				col[y]--
				diagonal[x-y]--
				antiDiagonal[x+y]--
			}
		}
	}
	return ans
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	assert(gridIllumination(1, [][]int{{0, 0}, {4, 4}}, [][]int{{1, 1}, {1, 0}}), []int{1, 0})
	assert(gridIllumination(5, [][]int{{0, 0}, {4, 4}}, [][]int{{1, 1}, {1, 1}}), []int{1, 1})
	assert(gridIllumination(5, [][]int{{0, 0}, {0, 4}}, [][]int{{0, 4}, {0, 1}, {1, 4}}), []int{1, 1, 0})
}
