/*
 * @Date: 2023-08-04
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-04
 * @FilePath: /algorithm/golang/980_unique_paths_iii/unique_paths_iii.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

var dir = [4][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}

func uniquePathsIII(grid [][]int) int {
	r, c := len(grid), len(grid[0])
	si, sj, st := 0, 0, 0
	memo := map[int]int{}
	for i := 0; i < r; i++ {
		for j := 0; j < c; j++ {
			if grid[i][j] == 0 || grid[i][j] == 2 {
				st |= (1 << (i*c + j))
			} else if grid[i][j] == 1 {
				si, sj = i, j
			}
		}
	}

	var dp func(i int, j int, st int) int
	dp = func(i int, j int, st int) int {
		if grid[i][j] == 2 {
			if st == 0 {
				return 1
			}
			return 0
		}
		key := ((i*c + j) << (r * c)) + st
		res, ok := memo[key]
		if !ok {
			res = 0
			for k := 0; k < 4; k++ {
				ni, nj := i+dir[k][0], j+dir[k][1]
				if ni >= 0 && ni < r && nj >= 0 && nj < c && (st&(1<<(ni*c+nj))) > 0 {
					res += dp(ni, nj, st^(1<<(ni*c+nj)))
				}
			}
			memo[key] = res
		}
		return res
	}
	return dp(si, sj, st)
}

func main() {
	tests := []struct {
		grid [][]int
		ans  int
	}{
		{[][]int{{1, 0, 0, 0}, {0, 0, 0, 0}, {0, 0, 2, -1}}, 2},
		{[][]int{{1, 0, 0, 0}, {0, 0, 0, 0}, {0, 0, 0, 2}}, 4},
		{[][]int{{0, 1}, {2, 0}}, 0},
	}

	for _, item := range tests {
		assert.Equal(&testing.T{}, item.ans, uniquePathsIII(item.grid))
	}
}
