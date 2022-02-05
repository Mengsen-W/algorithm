/*
 * @Date: 2022-02-05 02:08:28
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-05 02:20:30
 */

package main

var dirs = []struct{ x, y int }{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}

func getMaximumGold(grid [][]int) (ans int) {
	var dfs func(x, y, gold int)
	dfs = func(x, y, gold int) {
		gold += grid[x][y]
		if gold > ans {
			ans = gold
		}

		rec := grid[x][y]
		grid[x][y] = 0
		for _, d := range dirs {
			nx, ny := x+d.x, y+d.y
			if 0 <= nx && nx < len(grid) && 0 <= ny && ny < len(grid[nx]) && grid[nx][ny] > 0 {
				dfs(nx, ny, gold)
			}
		}
		grid[x][y] = rec
	}

	for i, row := range grid {
		for j, gold := range row {
			if gold > 0 {
				dfs(i, j, 0)
			}
		}
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(getMaximumGold(
		[][]int{{0, 6, 0}, {5, 8, 7}, {0, 9, 0}}) == 24)
	assert(getMaximumGold(
		[][]int{{1, 0, 7}, {2, 0, 6}, {3, 4, 5}, {0, 3, 0}, {9, 0, 20}}) == 28)
}
