/*
 * @Date: 2023-02-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-05
 * @FilePath: /algorithm/golang/1210_minimum_moves/minimum_moves.go
 */

package main

func minimumMoves(grid [][]int) int {
	n := len(grid)
	dist := make([][][2]int, n)
	for i := range dist {
		dist[i] = make([][2]int, n)
		for j := range dist[i] {
			dist[i][j] = [2]int{-1, -1}
		}
	}
	dist[0][0][0] = 0
	queue := [][3]int{{0, 0, 0}}

	for len(queue) > 0 {
		arr := queue[0]
		queue = queue[1:]
		x := arr[0]
		y := arr[1]
		status := arr[2]
		if status == 0 {
			// 向右移动一个单元格
			if y+2 < n && dist[x][y+1][0] == -1 && grid[x][y+2] == 0 {
				dist[x][y+1][0] = dist[x][y][0] + 1
				queue = append(queue, [3]int{x, y + 1, 0})
			}
			// 向下移动一个单元格
			if x+1 < n && dist[x+1][y][0] == -1 && grid[x+1][y] == 0 && grid[x+1][y+1] == 0 {
				dist[x+1][y][0] = dist[x][y][0] + 1
				queue = append(queue, [3]int{x + 1, y, 0})
			}
			// 顺时针旋转 90 度
			if x+1 < n && y+1 < n && dist[x][y][1] == -1 && grid[x+1][y] == 0 && grid[x+1][y+1] == 0 {
				dist[x][y][1] = dist[x][y][0] + 1
				queue = append(queue, [3]int{x, y, 1})
			}
		} else {
			// 向右移动一个单元格
			if y+1 < n && dist[x][y+1][1] == -1 && grid[x][y+1] == 0 && grid[x+1][y+1] == 0 {
				dist[x][y+1][1] = dist[x][y][1] + 1
				queue = append(queue, [3]int{x, y + 1, 1})
			}
			// 向下移动一个单元格
			if x+2 < n && dist[x+1][y][1] == -1 && grid[x+2][y] == 0 {
				dist[x+1][y][1] = dist[x][y][1] + 1
				queue = append(queue, [3]int{x + 1, y, 1})
			}
			// 逆时针旋转 90 度
			if x+1 < n && y+1 < n && dist[x][y][0] == -1 && grid[x][y+1] == 0 && grid[x+1][y+1] == 0 {
				dist[x][y][0] = dist[x][y][1] + 1
				queue = append(queue, [3]int{x, y, 0})
			}
		}
	}

	return dist[n-1][n-2][0]
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		grid := [][]int{{0, 0, 0, 0, 0, 1}, {1, 1, 0, 0, 1, 0}, {0, 0, 0, 0, 1, 1}, {0, 0, 1, 0, 1, 0}, {0, 1, 1, 0, 0, 0}, {0, 1, 1, 0, 0, 0}}
		ans := 11
		assert(minimumMoves(grid) == ans)
	}

	{
		grid := [][]int{{0, 0, 1, 1, 1, 1}, {0, 0, 0, 0, 1, 1}, {1, 1, 0, 0, 0, 1}, {1, 1, 1, 0, 0, 1}, {1, 1, 1, 0, 0, 1}, {1, 1, 1, 0, 0, 0}}
		ans := 9
		assert(minimumMoves(grid) == ans)
	}
}
