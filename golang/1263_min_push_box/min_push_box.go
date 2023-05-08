/*
 * @Date: 2023-05-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-08
 * @FilePath: /algorithm/golang/1263_min_push_box/min_push_box.go
 */

// Package main ...
package main

func minPushBox(grid [][]byte) int {
	m, n := len(grid), len(grid[0])
	var sx, sy, bx, by int // 玩家、箱子的初始位置
	for x := 0; x < m; x++ {
		for y := 0; y < n; y++ {
			if grid[x][y] == 'S' {
				sx, sy = x, y
			} else if grid[x][y] == 'B' {
				bx, by = x, y
			}
		}
	}

	ok := func(x, y int) bool { // 不越界且不在墙上
		return x >= 0 && x < m && y >= 0 && y < n && grid[x][y] != '#'
	}
	d := []int{0, -1, 0, 1, 0}

	dp := make([][]int, m*n)
	for i := 0; i < m*n; i++ {
		dp[i] = make([]int, m*n)
		for j := 0; j < m*n; j++ {
			dp[i][j] = 0x3f3f3f3f
		}
	}
	dp[sx*n+sy][bx*n+by] = 0 // 初始状态的推动次数为 0
	q := [][2]int{{sx*n + sy, bx*n + by}}
	for len(q) > 0 {
		q1 := [][2]int{}
		for len(q) > 0 {
			s1, b1 := q[0][0], q[0][1]
			q = q[1:]
			sx1, sy1, bx1, by1 := s1/n, s1%n, b1/n, b1%n
			if grid[bx1][by1] == 'T' { // 箱子已被推到目标处
				return dp[s1][b1]
			}
			for i := 0; i < 4; i++ { // 玩家向四个方向移动到另一个状态
				sx2, sy2 := sx1+d[i], sy1+d[i+1]
				s2 := sx2*n + sy2
				if !ok(sx2, sy2) { // 玩家位置不合法
					continue
				}
				if bx1 == sx2 && by1 == sy2 { // 推动箱子
					bx2, by2 := bx1+d[i], by1+d[i+1]
					b2 := bx2*n + by2
					if !ok(bx2, by2) || dp[s2][b2] <= dp[s1][b1]+1 { // 箱子位置不合法 或 状态已访问
						continue
					}
					dp[s2][b2] = dp[s1][b1] + 1
					q1 = append(q1, [2]int{s2, b2})
				} else {
					if dp[s2][b1] <= dp[s1][b1] { // 状态已访问
						continue
					}
					dp[s2][b1] = dp[s1][b1]
					q = append(q, [2]int{s2, b1})
				}
			}
		}
		q = q1
	}
	return -1
}

func main() {
	assert := func(a bool) {
		if !a {
			panic("Not Passed")
		}
	}

	{
		grid := [][]byte{{'#', '#', '#', '#', '#', '#'}, {'#', 'T', '#', '#', '#', '#'}, {'#', '.', '.', 'B', '.', '#'}, {'#', '.', '#', '#', '.', '#'}, {'#', '.', '.', '.', 'S', '#'}, {'#', '#', '#', '#', '#', '#'}}
		ans := 3
		assert(minPushBox(grid) == ans)
	}

	{
		grid := [][]byte{{'#', '#', '#', '#', '#', '#'}, {'#', 'T', '#', '#', '#', '#'}, {'#', '.', '.', 'B', '.', '#'}, {'#', '#', '#', '#', '.', '#'}, {'#', '.', '.', '.', 'S', '#'}, {'#', '#', '#', '#', '#', '#'}}
		ans := -1
		assert(minPushBox(grid) == ans)
	}

	{
		grid := [][]byte{{'#', '#', '#', '#', '#', '#'}, {'#', 'T', '.', '.', '#', '#'}, {'#', '.', '#', 'B', '.', '#'}, {'#', '.', '.', '.', '.', '#'}, {'#', '.', '.', '.', 'S', '#'}, {'#', '#', '#', '#', '#', '#'}}
		ans := 5
		assert(minPushBox(grid) == ans)
	}
}
