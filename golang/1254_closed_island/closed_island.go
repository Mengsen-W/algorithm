// Package main ...
package main

import (
	"fmt"
	"reflect"
)

func closedIsland(grid [][]int) int {
	m, n := len(grid), len(grid[0])
	ans := 0

	var dfs func(x, y int) bool
	dfs = func(x, y int) bool {
		if x < 0 || y < 0 || x >= m || y >= n {
			return false
		}
		if grid[x][y] != 0 {
			return true
		}

		grid[x][y] = -1
		ret1, ret2, ret3, ret4 := dfs(x-1, y), dfs(x+1, y), dfs(x, y-1), dfs(x, y+1)
		return ret1 && ret2 && ret3 && ret4
	}

	for i := range grid {
		for j := range grid[i] {
			if grid[i][j] == 0 && dfs(i, j) {
				ans++
			}
		}
	}

	return ans
}

func main() {
	tests := []struct {
		grid [][]int
		ans  int
	}{
		{[][]int{
			{1, 1, 1, 1, 1, 1, 1, 0},
			{1, 0, 0, 0, 0, 1, 1, 0},
			{1, 0, 1, 0, 1, 1, 1, 0},
			{1, 0, 0, 0, 0, 1, 0, 1},
			{1, 1, 1, 1, 1, 1, 1, 0},
		}, 2},
		{[][]int{{0, 0, 1, 0, 0}, {0, 1, 0, 1, 0}, {0, 1, 1, 1, 0}}, 1},
		{[][]int{
			{1, 1, 1, 1, 1, 1, 1},
			{1, 0, 0, 0, 0, 0, 1},
			{1, 0, 1, 1, 1, 0, 1},
			{1, 0, 1, 0, 1, 0, 1},
			{1, 0, 1, 1, 1, 0, 1},
			{1, 0, 0, 0, 0, 0, 1},
			{1, 1, 1, 1, 1, 1, 1},
		}, 2},
	}
	for index, test := range tests {
		if reflect.DeepEqual(closedIsland(test.grid), test.ans) {
			fmt.Println(index)
		}
	}
}
