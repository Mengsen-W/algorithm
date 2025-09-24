// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func largestPathValue(colors string, edges [][]int) int {
	n := len(colors)
	// 邻接表
	g := make([][]int, n)
	// 节点的入度统计，用于找出拓扑排序中最开始的节点
	indeg := make([]int, n)
	for _, edge := range edges {
		indeg[edge[1]]++
		g[edge[0]] = append(g[edge[0]], edge[1])
	}

	// 记录拓扑排序过程中遇到的节点个数
	// 如果最终 found 的值不为 n，说明图中存在环
	found := 0
	f := make([][26]int, n)
	q := []int{}
	for i := 0; i < n; i++ {
		if indeg[i] == 0 {
			q = append(q, i)
		}
	}

	for len(q) > 0 {
		found++
		u := q[0]
		q = q[1:]
		// 将节点 u 对应的颜色增加 1
		f[u][colors[u]-'a']++
		// 枚举 u 的后继节点 v
		for _, v := range g[u] {
			indeg[v]--
			// 将 f(v,c) 更新为其与 f(u,c) 的较大值
			for c := 0; c < 26; c++ {
				f[v][c] = max(f[v][c], f[u][c])
			}
			if indeg[v] == 0 {
				q = append(q, v)
			}
		}
	}
	if found != n {
		return -1
	}
	ans := 0
	for i := 0; i < n; i++ {
		for c := 0; c < 26; c++ {
			ans = max(ans, f[i][c])
		}
	}
	return ans
}

func main() {
	tests := []struct {
		colors string
		edges  [][]int
		ans    int
	}{
		{"abaca", [][]int{{0, 1}, {0, 2}, {2, 3}, {3, 4}}, 3},
		{"a", [][]int{{0, 0}}, -1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, largestPathValue(test.colors, test.edges), index)
	}
}
