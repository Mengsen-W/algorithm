// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximumDetonation(bombs [][]int) int {
	n := len(bombs)
	// 判断炸弹 u 能否引爆炸弹 v
	isConnected := func(u, v int) bool {
		dx := bombs[u][0] - bombs[v][0]
		dy := bombs[u][1] - bombs[v][1]
		return int64(bombs[u][2]*bombs[u][2]) >= int64(dx*dx+dy*dy)
	}

	// 维护引爆关系有向图
	edges := make(map[int][]int)
	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			if i != j && isConnected(i, j) {
				edges[i] = append(edges[i], j)
			}
		}
	}
	res := 0 // 最多引爆数量
	for i := 0; i < n; i++ {
		// 遍历每个炸弹，广度优先搜索计算该炸弹可引爆的数量，并维护最大值
		visited := make([]int, n)
		cnt := 1
		q := []int{}
		q = append(q, i)
		visited[i] = 1
		for len(q) > 0 {
			cidx := q[0]
			q = q[1:]
			for _, nidx := range edges[cidx] {
				if visited[nidx] != 0 {
					continue
				}
				cnt++
				q = append(q, nidx)
				visited[nidx] = 1
			}
		}
		if cnt > res {
			res = cnt
		}
	}
	return res
}

func main() {
	tests := []struct {
		bombs [][]int
		ans   int
	}{
		{[][]int{{2, 1, 3}, {6, 1, 4}}, 2},
		{[][]int{{1, 1, 5}, {10, 10, 5}}, 1},
		{[][]int{{1, 2, 3}, {2, 3, 1}, {3, 4, 2}, {4, 5, 3}, {5, 6, 4}}, 5},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maximumDetonation(test.bombs), index)
	}
}
