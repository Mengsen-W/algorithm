/*
 * @Date: 2022-10-25
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-25
 * @FilePath: /algorithm/934_shortest_bridge/shortest_bridge.go
 */

package main

func shortestBridge(grid [][]int) (step int) {
	type pair struct{ x, y int }
	dirs := []pair{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}
	n := len(grid)
	for i, row := range grid {
		for j, v := range row {
			if v != 1 {
				continue
			}
			q := []pair{}
			var dfs func(int, int)
			dfs = func(i, j int) {
				grid[i][j] = -1
				q = append(q, pair{i, j})
				for _, d := range dirs {
					x, y := i+d.x, j+d.y
					if 0 <= x && x < n && 0 <= y && y < n && grid[x][y] == 1 {
						dfs(x, y)
					}
				}
			}
			dfs(i, j)

			for {
				tmp := q
				q = nil
				for _, p := range tmp {
					for _, d := range dirs {
						x, y := p.x+d.x, p.y+d.y
						if 0 <= x && x < n && 0 <= y && y < n {
							if grid[x][y] == 1 {
								return
							}
							if grid[x][y] == 0 {
								grid[x][y] = -1
								q = append(q, pair{x, y})
							}
						}
					}
				}
				step++
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

	{
		grid := [][]int{{0, 1}, {1, 0}}
		ans := 1
		assert(shortestBridge(grid) == ans)
	}

	{
		grid := [][]int{{0, 1, 0}, {0, 0, 0}, {0, 0, 1}}
		ans := 2
		assert(shortestBridge(grid) == ans)
	}

	{
		grid := [][]int{{1, 1, 1, 1, 1}, {1, 0, 0, 0, 1}, {1, 0, 1, 0, 1}, {1, 0, 0, 0, 1}, {1, 1, 1, 1, 1}}
		ans := 1
		assert(shortestBridge(grid) == ans)
	}
}
