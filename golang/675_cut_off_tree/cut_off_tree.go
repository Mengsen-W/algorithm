/*
 * @Date: 2022-05-23 18:57:14
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-23 19:03:48
 * @FilePath: /algorithm/675_cut_off_tree/cut_off_tree.go
 */

package main

import "sort"

var dir4 = []struct{ x, y int }{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}

func cutOffTree(forest [][]int) (ans int) {
	type pair struct{ dis, x, y int }
	trees := []pair{}
	for i, row := range forest {
		for j, h := range row {
			if h > 1 {
				trees = append(trees, pair{h, i, j})
			}
		}
	}
	sort.Slice(trees, func(i, j int) bool { return trees[i].dis < trees[j].dis })

	bfs := func(sx, sy, tx, ty int) int {
		m, n := len(forest), len(forest[0])
		vis := make([][]bool, m)
		for i := range vis {
			vis[i] = make([]bool, n)
		}
		vis[sx][sy] = true
		q := []pair{{0, sx, sy}}
		for len(q) > 0 {
			p := q[0]
			q = q[1:]
			if p.x == tx && p.y == ty {
				return p.dis
			}
			for _, d := range dir4 {
				if x, y := p.x+d.x, p.y+d.y; 0 <= x && x < m && 0 <= y && y < n && !vis[x][y] && forest[x][y] > 0 {
					vis[x][y] = true
					q = append(q, pair{p.dis + 1, x, y})
				}
			}
		}
		return -1
	}

	preX, preY := 0, 0
	for _, t := range trees {
		d := bfs(preX, preY, t.x, t.y)
		if d < 0 {
			return -1
		}
		ans += d
		preX, preY = t.x, t.y
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(cutOffTree([][]int{{1, 2, 3}, {0, 0, 4}, {7, 6, 5}}) == 6)
	assert(cutOffTree([][]int{{1, 2, 3}, {0, 0, 0}, {7, 6, 5}}) == -1)
	assert(cutOffTree([][]int{{2, 3, 4}, {0, 0, 5}, {8, 7, 6}}) == 6)
}
