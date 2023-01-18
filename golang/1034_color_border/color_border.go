/*
 * @Date: 2021-12-07 00:56:37
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-07 01:17:21
 */

package main

import "reflect"

var dirs = []struct{ x, y int }{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}

func colorBorder_dfs(grid [][]int, row, col, color int) [][]int {
	m, n := len(grid), len(grid[0])
	type point struct{ x, y int }
	borders := []point{}
	originalColor := grid[row][col]
	vis := make([][]bool, m)
	for i := range vis {
		vis[i] = make([]bool, n)
	}

	var dfs func(int, int)
	dfs = func(x, y int) {
		vis[x][y] = true
		isBorder := false
		for _, dir := range dirs {
			nx, ny := x+dir.x, y+dir.y
			if !(0 <= nx && nx < m && 0 <= ny && ny < n && grid[nx][ny] == originalColor) {
				isBorder = true
			} else if !vis[nx][ny] {
				vis[nx][ny] = true
				dfs(nx, ny)
			}
		}
		if isBorder {
			borders = append(borders, point{x, y})
		}
	}
	dfs(row, col)

	for _, p := range borders {
		grid[p.x][p.y] = color
	}
	return grid
}

func colorBorder_bfs(grid [][]int, row, col, color int) [][]int {
	m, n := len(grid), len(grid[0])
	type point struct{ x, y int }
	borders := []point{}
	originalColor := grid[row][col]
	vis := make([][]bool, m)
	for i := range vis {
		vis[i] = make([]bool, n)
	}

	q := []point{{row, col}}
	vis[row][col] = true
	for len(q) > 0 {
		p := q[0]
		q = q[1:]
		x, y := p.x, p.y
		isBorder := false
		for _, dir := range dirs {
			nx, ny := x+dir.x, y+dir.y
			if !(0 <= nx && nx < m && 0 <= ny && ny < n && grid[nx][ny] == originalColor) {
				isBorder = true
			} else if !vis[nx][ny] {
				vis[nx][ny] = true
				q = append(q, point{nx, ny})
			}
		}
		if isBorder {
			borders = append(borders, point{x, y})
		}
	}

	for _, p := range borders {
		grid[p.x][p.y] = color
	}
	return grid
}

func main() {
	assert := func(a, b [][]int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	assert(colorBorder_dfs([][]int{{1, 1}, {1, 2}}, 0, 0, 3), [][]int{{3, 3}, {3, 2}})
	assert(colorBorder_bfs([][]int{{1, 1}, {1, 2}}, 0, 0, 3), [][]int{{3, 3}, {3, 2}})
	assert(colorBorder_dfs([][]int{{1, 2, 2}, {2, 3, 2}}, 0, 1, 3), [][]int{{1, 3, 3}, {2, 3, 3}})
	assert(colorBorder_bfs([][]int{{1, 2, 2}, {2, 3, 2}}, 0, 1, 3), [][]int{{1, 3, 3}, {2, 3, 3}})
	assert(colorBorder_dfs([][]int{{1, 1, 1}, {1, 1, 1}, {1, 1, 1}}, 1, 1, 2), [][]int{{2, 2, 2}, {2, 1, 2}, {2, 2, 2}})
	assert(colorBorder_bfs([][]int{{1, 1, 1}, {1, 1, 1}, {1, 1, 1}}, 1, 1, 2), [][]int{{2, 2, 2}, {2, 1, 2}, {2, 2, 2}})
}
