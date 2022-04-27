/*
 * @Date: 2022-04-27 09:41:45
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-27 09:55:37
 * @FilePath: /algorithm/417_pacific_atlantic/pacific_atlantic.go
 */

package main

import "reflect"

type pair struct{ x, y int }

var dirs = []pair{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}

func pacificAtlantic(heights [][]int) (ans [][]int) {
	m, n := len(heights), len(heights[0])
	pacific := make([][]bool, m)
	atlantic := make([][]bool, m)
	for i := range pacific {
		pacific[i] = make([]bool, n)
		atlantic[i] = make([]bool, n)
	}

	bfs := func(x, y int, ocean [][]bool) {
		if ocean[x][y] {
			return
		}
		ocean[x][y] = true
		q := []pair{{x, y}}
		for len(q) > 0 {
			p := q[0]
			q = q[1:]
			for _, d := range dirs {
				if x, y := p.x+d.x, p.y+d.y; 0 <= x && x < m && 0 <= y && y < n && !ocean[x][y] && heights[x][y] >= heights[p.x][p.y] {
					ocean[x][y] = true
					q = append(q, pair{x, y})
				}
			}
		}
	}
	for i := 0; i < m; i++ {
		bfs(i, 0, pacific)
	}
	for j := 1; j < n; j++ {
		bfs(0, j, pacific)
	}
	for i := 0; i < m; i++ {
		bfs(i, n-1, atlantic)
	}
	for j := 0; j < n-1; j++ {
		bfs(m-1, j, atlantic)
	}

	for i, row := range pacific {
		for j, ok := range row {
			if ok && atlantic[i][j] {
				ans = append(ans, []int{i, j})
			}
		}
	}
	return
}

func main() {
	assert := func(a, b [][]int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		heights := [][]int{{1, 2, 2, 3, 5}, {3, 2, 3, 4, 4}, {2, 4, 5, 3, 1}, {6, 7, 1, 4, 5}, {5, 1, 1, 2, 4}}
		ans := [][]int{{0, 4}, {1, 3}, {1, 4}, {2, 2}, {3, 0}, {3, 1}, {4, 0}}
		assert(pacificAtlantic(heights), ans)
	}

	{
		heights := [][]int{{2, 1}, {1, 2}}
		ans := [][]int{{0, 0}, {0, 1}, {1, 0}, {1, 1}}
		assert(pacificAtlantic(heights), ans)
	}
}
