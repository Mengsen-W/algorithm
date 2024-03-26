/*
 * @Date: 2024-03-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-26
 * @FilePath: /algorithm/golang/2642_Graph/Graph.go
 */

// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

const inf = math.MaxInt / 3 // 防止更新最短路时加法溢出

type Graph [][]int

func Constructor(n int, edges [][]int) Graph {
	f := make(Graph, n) // 邻接矩阵
	for i := range f {
		f[i] = make([]int, n)
		for j := range f[i] {
			if j != i {
				f[i][j] = inf
			}
		}
	}
	for _, e := range edges {
		f[e[0]][e[1]] = e[2] // 添加一条边（题目保证没有重边和自环）
	}
	for k := range f {
		for i := range f {
			if f[i][k] == inf {
				continue
			}
			for j := range f {
				f[i][j] = min(f[i][j], f[i][k]+f[k][j])
			}
		}
	}
	return f
}

func (f Graph) AddEdge(e []int) {
	x, y, w := e[0], e[1], e[2]
	if w >= f[x][y] { // 无需更新
		return
	}
	for i := range f {
		for j := range f {
			f[i][j] = min(f[i][j], f[i][x]+w+f[y][j])
		}
	}
}

func (f Graph) ShortestPath(start, end int) int {
	ans := f[start][end]
	if ans == inf {
		return -1
	}
	return ans
}

func main() {
	g := Constructor(4, [][]int{{0, 2, 5}, {0, 1, 2}, {1, 2, 1}, {3, 0, 3}})
	assert.Equal(&testing.T{}, 6, g.ShortestPath(3, 2))
	// 返回 6 。从 3 到 2 的最短路径如第一幅图所示：3 -> 0 -> 1 -> 2 ，总代价为 3 + 2 + 1 = 6 。
	assert.Equal(&testing.T{}, -1, g.ShortestPath(0, 3))
	// 返回 -1 。没有从 0 到 3 的路径。
	g.AddEdge([]int{1, 3, 4}) // 添加一条节点 1 到节点 3 的边，得到第二幅图。
	assert.Equal(&testing.T{}, 6, g.ShortestPath(0, 3))
	// 返回 6 。从 0 到 3 的最短路径为 0 -> 1 -> 3 ，总代价为 2 + 4 = 6 。
}
