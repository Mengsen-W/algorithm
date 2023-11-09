/*
 * @Date: 2023-11-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-09
 * @FilePath: /algorithm/golang/2258_maximum_minutes/maximum_minutes.go
 */

package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

var (
	dirs     = [][]int{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}
	INF  int = 1e9
)

func maximumMinutes(grid [][]int) int {
	m, n := len(grid), len(grid[0])
	fireTime := make([][]int, m)
	for i := 0; i < m; i++ {
		fireTime[i] = make([]int, n)
		for j := 0; j < n; j++ {
			fireTime[i][j] = INF
		}
	}

	bfs := func() {
		q := [][]int{}
		for i := 0; i < m; i++ {
			for j := 0; j < n; j++ {
				if grid[i][j] == 1 {
					q = append(q, []int{i, j})
					fireTime[i][j] = 0
				}
			}
		}

		time := 1
		for len(q) > 0 {
			tmp := q
			q = [][]int{}
			for _, p := range tmp {
				cx, cy := p[0], p[1]
				for _, d := range dirs {
					nx, ny := cx+d[0], cy+d[1]
					if nx >= 0 && ny >= 0 && nx < m && ny < n {
						if grid[nx][ny] == 2 || fireTime[nx][ny] != INF {
							continue
						}
						q = append(q, []int{nx, ny})
						fireTime[nx][ny] = time
					}
				}
			}
			time += 1
		}
	}

	getArriveTime := func(stayTime int) int {
		visit := make([][]bool, m)
		for i := 0; i < m; i++ {
			visit[i] = make([]bool, n)
		}
		q := [][]int{}
		q = append(q, []int{0, 0, stayTime})
		for len(q) > 0 {
			tmp := q
			q = [][]int{}

			for _, p := range tmp {
				cx, cy, time := p[0], p[1], p[2]
				for _, d := range dirs {
					nx, ny := cx+d[0], cy+d[1]
					if nx >= 0 && ny >= 0 && nx < m && ny < n {
						if visit[nx][ny] || grid[nx][ny] == 2 {
							continue
						}
						/* 到达安全屋 */
						if nx == m-1 && ny == n-1 {
							return time + 1
						}
						/* 火未到达当前位置 */
						if fireTime[nx][ny] > time+1 {
							q = append(q, []int{nx, ny, time + 1})
							visit[nx][ny] = true
						}
					}
				}
			}
		}
		return -1
	}

	/* 通过 bfs 求出每个格子着火的时间 */
	bfs()
	/* 找到起点到终点的最短路径 */
	arriveTime := getArriveTime(0)
	/* 安全屋不可达 */
	if arriveTime < 0 {
		return -1
	}
	/* 火不会到达安全屋 */
	if fireTime[m-1][n-1] == INF {
		return 1e9
	}
	ans := fireTime[m-1][n-1] - arriveTime
	if getArriveTime(ans) >= 0 {
		return ans
	}
	return ans - 1
}

func main() {
	tests := []struct {
		grid [][]int
		ans  int
	}{
		{
			[][]int{
				{0, 2, 0, 0, 0, 0, 0},
				{0, 0, 0, 2, 2, 1, 0},
				{0, 2, 0, 0, 1, 2, 0},
				{0, 0, 2, 2, 2, 0, 2},
				{0, 0, 0, 0, 0, 0, 0},
			},
			3,
		},
		{[][]int{{0, 0, 0, 0}, {0, 1, 2, 0}, {0, 2, 0, 0}}, -1},
		{[][]int{{0, 0, 0}, {2, 2, 0}, {1, 2, 0}}, 1000000000},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maximumMinutes(test.grid), index)
	}
}
