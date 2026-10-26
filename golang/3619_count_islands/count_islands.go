// Package main ...
package main

import "strconv"

func countIslands(grid [][]int, k int) (ans int) {
	dirs := []struct{ x, y int }{{0, -1}, {0, 1}, {-1, 0}, {1, 0}}
	m, n := len(grid), len(grid[0])

	var dfs func(int, int) int
	dfs = func(i, j int) int {
		res := grid[i][j]
		grid[i][j] = 0 // 标记 (i,j) 访问过
		for _, d := range dirs {
			x, y := i+d.x, j+d.y
			if 0 <= x && x < m && 0 <= y && y < n && grid[x][y] > 0 {
				res += dfs(x, y)
			}
		}
		return res
	}

	for i, row := range grid {
		for j, x := range row {
			if x > 0 && dfs(i, j)%k == 0 {
				ans++
			}
		}
	}
	return
}

func main() {
	tests := []struct {
		grid [][]int
		k    int
		ans  int
	}{
		{
			[][]int{
				{0, 2, 1, 0, 0},
				{0, 5, 0, 0, 5},
				{0, 0, 1, 0, 0},
				{0, 1, 4, 7, 0},
				{0, 2, 0, 0, 8},
			},
			5,
			2,
		},
		{
			[][]int{
				{3, 0, 3, 0},
				{0, 3, 0, 3},
				{3, 0, 3, 0},
			},
			3,
			6,
		},
	}

	for _, test := range tests {
		if got := countIslands(test.grid, test.k); got != test.ans {
			panic("got " + strconv.Itoa(got) + " want " + strconv.Itoa(test.ans))
		}
	}
}
