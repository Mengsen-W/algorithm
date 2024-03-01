/*
 * @Date: 2024-02-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-29
 * @FilePath: /algorithm/golang/2581_root_count/root_count.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func rootCount(edges [][]int, guesses [][]int, k int) int {
	n := len(edges) + 1
	g := make([][]int, n)
	st := make(map[int64]int)

	h := func(x, y int) int64 {
		return int64(x)<<20 | int64(y)
	}

	for _, v := range edges {
		g[v[0]] = append(g[v[0]], v[1])
		g[v[1]] = append(g[v[1]], v[0])
	}
	for _, v := range guesses {
		st[h(v[0], v[1])] = 1
	}

	cnt, res := 0, 0
	var dfs func(int, int)
	dfs = func(x, fat int) {
		for _, y := range g[x] {
			if y == fat {
				continue
			}
			if st[h(x, y)] == 1 {
				cnt++
			}
			dfs(y, x)
		}
	}
	dfs(0, -1)

	var redfs func(int, int, int)
	redfs = func(x, fat, cnt int) {
		if cnt >= k {
			res++
		}
		for _, y := range g[x] {
			if y == fat {
				continue
			}
			redfs(y, x, cnt-st[h(x, y)]+st[h(y, x)])
		}
	}
	redfs(0, -1, cnt)
	return res
}

func main() {
	tests := []struct {
		edges   [][]int
		guesses [][]int
		k       int
		ans     int
	}{
		{[][]int{{0, 1}, {1, 2}, {1, 3}, {4, 2}}, [][]int{{1, 3}, {0, 1}, {1, 0}, {2, 4}}, 3, 3},
		{[][]int{{0, 1}, {1, 2}, {2, 3}, {3, 4}}, [][]int{{1, 0}, {3, 4}, {2, 1}, {3, 2}}, 1, 5},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, rootCount(test.edges, test.guesses, test.k), index)
	}
}
