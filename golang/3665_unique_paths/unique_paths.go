// Package main ...
package main

import (
	"fmt"
)

func uniquePaths(grid [][]int) int {
	const MOD = 1000000007
	m, n := len(grid), len(grid[0])

	// f[j][0]：通过向下移动进入当前列
	// f[j][1]：通过向右移动进入当前列
	f := make([][2]int64, n)
	f[0][0] = 1

	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			if i > 0 {
				if grid[i-1][j] == 0 {
					f[j][0] = (f[j][0] + f[j][1]) % MOD
				} else {
					f[j][0] = f[j][1]
				}
			}

			if j > 0 {
				if grid[i][j-1] == 0 {
					f[j][1] = (f[j-1][0] + f[j-1][1]) % MOD
				} else {
					f[j][1] = f[j-1][0]
				}
			}
		}
	}

	return int((f[n-1][0] + f[n-1][1]) % MOD)
}

func main() {
	tests := []struct {
		grid [][]int
		ans  int
	}{
		{ [][]int{{0, 1, 0}, {0, 0, 1}, {1, 0, 0}}, 5},
		{ [][]int{{0, 0}, {0, 0}}, 2},
		{ [][]int{{0, 1, 1}, {1, 1, 0}}, 1},
	}

	for index, test := range tests {
		if ans := uniquePaths(test.grid); ans != test.ans {
			fmt.Println(index)
		}
	}
}
