// Package main ...
package main

import (
	"container/list"
	"fmt"
	"math"
)

func findSafeWalk(grid [][]int, health int) bool {
	m, n := len(grid), len(grid[0])
	dirs := [][2]int{{0, 1}, {0, -1}, {1, 0}, {-1, 0}}

	dis := make([][]int, m)
	for i := range dis {
		dis[i] = make([]int, n)
		for j := range dis[i] {
			dis[i][j] = math.MaxInt32
		}
	}

	q := list.New()
	q.PushFront([2]int{0, 0})
	dis[0][0] = grid[0][0]

	for q.Len() > 0 {
		cur := q.Remove(q.Front()).([2]int)
		cx, cy := cur[0], cur[1]
		// 第一次出队时，保证是最短距离
		if cx == m-1 && cy == n-1 {
			return true
		}

		for _, dir := range dirs {
			nx, ny := cx+dir[0], cy+dir[1]
			if nx < 0 || ny < 0 || nx >= m || ny >= n {
				continue
			}

			cost := dis[cx][cy] + grid[nx][ny]
			// 剪枝：新距离不满足健康要求
			if cost >= health {
				continue
			}

			if cost < dis[nx][ny] {
				dis[nx][ny] = cost
				if grid[nx][ny] == 0 {
					q.PushFront([2]int{nx, ny})
				} else {
					q.PushBack([2]int{nx, ny})
				}
			}
		}
	}
	return false
}

func main() {
	tests := []struct {
		grid   [][]int
		health int
		ans    bool
	}{
		{[][]int{{0, 1, 0, 0, 0}, {0, 1, 0, 1, 0}, {0, 0, 0, 1, 0}}, 1, true},
		{[][]int{{0, 1, 1, 0, 0, 0}, {1, 0, 1, 0, 0, 0}, {0, 1, 1, 1, 0, 1}, {0, 0, 1, 0, 1, 0}}, 3, false},
		{[][]int{{1, 1, 1}, {1, 0, 1}, {1, 1, 1}}, 5, true},
	}

	for index, test := range tests {
		if res := findSafeWalk(test.grid, test.health); res != test.ans {
			fmt.Printf("test %d failed, expected %v, got %v\n", index, test.ans, res)
		}
	}
}
