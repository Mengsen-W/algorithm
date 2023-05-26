/*
 * @Date: 2023-05-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-26
 * @FilePath: /algorithm/golang/1091_shortest_path_binary_matrix/shortest_path_binary_matrix.go
 */

// Package main ...
package main

func shortestPathBinaryMatrix(grid [][]int) int {
	if grid[0][0] == 1 {
		return -1
	}
	n := len(grid)
	dist := make([][]int, n)
	for i := 0; i < n; i++ {
		dist[i] = make([]int, n)
		for j := 0; j < n; j++ {
			dist[i][j] = 0x3f3f3f3f
		}
	}
	q := [][2]int{{0, 0}}
	dist[0][0] = 1
	for len(q) > 0 {
		x, y := q[0][0], q[0][1]
		q = q[1:]
		if x == n-1 && y == n-1 {
			return dist[x][y]
		}
		for dx := -1; dx <= 1; dx++ {
			for dy := -1; dy <= 1; dy++ {
				if x+dx < 0 || x+dx >= n || y+dy < 0 || y+dy >= n { // 越界
					continue
				}
				if grid[x+dx][y+dy] == 1 || dist[x][y]+1 >= dist[x+dx][y+dy] { // 单元格值不为 0 或已被访问
					continue
				}
				dist[x+dx][y+dy] = dist[x][y] + 1
				q = append(q, [2]int{x + dx, y + dy})
			}
		}
	}
	return -1
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		grid := [][]int{{0, 1}, {1, 0}}
		ans := 2
		assert(shortestPathBinaryMatrix(grid) == ans)
	}

	{
		grid := [][]int{{0, 0, 0}, {1, 1, 0}, {1, 1, 0}}
		ans := 4
		assert(shortestPathBinaryMatrix(grid) == ans)
	}

	{
		grid := [][]int{{1, 0, 0}, {1, 1, 0}, {1, 1, 0}}
		ans := -1
		assert(shortestPathBinaryMatrix(grid) == ans)
	}
}
