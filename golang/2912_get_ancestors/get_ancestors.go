/*
 * @Date: 2024-04-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-05
 * @FilePath: /algorithm/golang/2912_get_ancestors/get_ancestors.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func getAncestors(n int, edges [][]int) [][]int {
	g := make([][]int, n)
	for _, e := range edges {
		x, y := e[0], e[1]
		g[x] = append(g[x], y)
	}

	ans := make([][]int, n)
	vis := make([]int, n)
	start := 0
	var dfs func(int)
	dfs = func(x int) {
		vis[x] = start + 1 // 避免重复访问
		for _, y := range g[x] {
			if vis[y] != start+1 {
				ans[y] = append(ans[y], start) // start 是访问到的点的祖先
				dfs(y)                         // 只递归没有访问过的点
			}
		}
	}
	for ; start < n; start++ {
		dfs(start) // 从 start 开始 DFS
	}
	return ans
}

func main() {
	tests := []struct {
		n     int
		edges [][]int
		ans   [][]int
	}{
		{
			8,
			[][]int{{0, 3}, {0, 4}, {1, 3}, {2, 4}, {2, 7}, {3, 5}, {3, 6}, {3, 7}, {4, 6}},
			[][]int{{}, {}, {}, {0, 1}, {0, 2}, {0, 1, 3}, {0, 1, 2, 3, 4}, {0, 1, 2, 3}},
		},
		{
			5,
			[][]int{{0, 1}, {0, 2}, {0, 3}, {0, 4}, {1, 2}, {1, 3}, {1, 4}, {2, 3}, {2, 4}, {3, 4}},
			[][]int{{}, {0}, {0, 1}, {0, 1, 2}, {0, 1, 2, 3}},
		},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, getAncestors(test.n, test.edges), index)
	}
}
