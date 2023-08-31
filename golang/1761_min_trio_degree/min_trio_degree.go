/*
 * @Date: 2023-08-31
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-31
 * @FilePath: /algorithm/golang/1761_min_trio_degree/min_trio_degree.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minTrioDegree(n int, edges [][]int) int {
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}
	g := make([][]int, n)
	degree := make([]int, n)
	for i := 0; i < n; i++ {
		g[i] = make([]int, n)
	}
	for _, edge := range edges {
		x, y := edge[0]-1, edge[1]-1
		g[x][y], g[y][x] = 1, 1
		degree[x]++
		degree[y]++
	}
	ans := 0x3f3f3f3f
	for i := 0; i < n; i++ {
		for j := i + 1; j < n; j++ {
			if g[i][j] != 1 {
				continue
			}
			for k := j + 1; k < n; k++ {
				if g[i][k] == 1 && g[j][k] == 1 {
					ans = min(ans, degree[i]+degree[j]+degree[k]-6)
				}
			}
		}
	}
	if ans == 0x3f3f3f3f {
		return -1
	}
	return ans
}

func main() {
	tests := []struct {
		n     int
		edges [][]int
		ans   int
	}{
		{6, [][]int{{1, 2}, {1, 3}, {3, 2}, {4, 1}, {5, 2}, {3, 6}}, 3},
		{7, [][]int{{1, 3}, {4, 1}, {4, 3}, {2, 5}, {5, 6}, {6, 7}, {7, 5}, {2, 6}}, 0},
	}

	for index, item := range tests {
		assert.Equal(&testing.T{}, item.ans, minTrioDegree(item.n, item.edges), index)
	}
}
