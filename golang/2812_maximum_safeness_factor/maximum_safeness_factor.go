// Package main ...
package main

import "fmt"

func maximumSafenessFactor(grid [][]int) int {
	n := len(grid)
	if grid[0][0] == 1 || grid[n-1][n-1] == 1 {
		return 0
	}

	dis := make([][]int, n)
	for i := range dis {
		dis[i] = make([]int, n)
		for j := range dis[i] {
			dis[i][j] = -1
		}
	}

	dirs := [][2]int{{-1, 0}, {1, 0}, {0, 1}, {0, -1}}
	q := make([][2]int, 0)

	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			if grid[i][j] == 1 {
				q = append(q, [2]int{i, j})
				dis[i][j] = 0
			}
		}
	}

	for len(q) > 0 {
		cx, cy := q[0][0], q[0][1]
		q = q[1:]
		for _, dir := range dirs {
			nx, ny := cx+dir[0], cy+dir[1]
			if nx < 0 || ny < 0 || nx >= n || ny >= n || dis[nx][ny] != -1 {
				continue
			}
			dis[nx][ny] = dis[cx][cy] + 1
			q = append(q, [2]int{nx, ny})
		}
	}

	check := func(limit int) bool {
		visit := make([][]bool, n)
		for i := range visit {
			visit[i] = make([]bool, n)
		}
		q := [][2]int{{0, 0}}
		visit[0][0] = true

		for len(q) > 0 {
			cx, cy := q[0][0], q[0][1]
			q = q[1:]
			if cx == n-1 && cy == n-1 {
				return true
			}
			for _, dir := range dirs {
				nx, ny := cx+dir[0], cy+dir[1]
				if nx < 0 || ny < 0 || nx >= n || ny >= n ||
					visit[nx][ny] || dis[nx][ny] < limit {
					continue
				}
				q = append(q, [2]int{nx, ny})
				visit[nx][ny] = true
			}
		}
		return false
	}

	lo, hi := 0, min(dis[0][0], dis[n-1][n-1])
	res := 0
	for lo <= hi {
		mid := (lo + hi) / 2
		if check(mid) {
			res = mid
			lo = mid + 1
		} else {
			hi = mid - 1
		}
	}
	return res
}

func main() {
	tests := []struct {
		grid     [][]int
		expected int
	}{
		{[][]int{{1, 0, 0}, {0, 0, 0}, {0, 0, 1}}, 0},
		{[][]int{{0, 0, 1}, {0, 0, 0}, {0, 0, 0}}, 2},
		{[][]int{{0, 0, 0, 1}, {0, 0, 0, 0}, {0, 0, 0, 0}, {1, 0, 0, 0}}, 2},
	}

	for _, test := range tests {
		if ans := maximumSafenessFactor(test.grid); ans != test.expected {
			fmt.Println("test failed", test.grid, ans, test.expected)
		}
	}
}
