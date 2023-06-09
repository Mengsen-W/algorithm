/*
 * @Date: 2023-06-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-09
 * @FilePath: /algorithm/golang/2699_modified_graph_edges/modified_graph_edges.go
 */

// Package main
package main

import (
	"fmt"
	"reflect"
)

func modifiedGraphEdges(n int, edges [][]int, source int, destination int, target int) [][]int {
	adjMatrix := make([][]int, n)
	for i := 0; i < n; i++ {
		adjMatrix[i] = make([]int, n)
		for j := 0; j < n; j++ {
			adjMatrix[i][j] = -1
		}
	}
	// 邻接矩阵中存储边的下标
	for i, e := range edges {
		u, v := e[0], e[1]
		adjMatrix[u][v], adjMatrix[v][u] = i, i
	}
	fromDestination := dijkstra(0, destination, edges, adjMatrix, target, nil)
	if fromDestination[source] > int64(target) {
		return nil
	}
	fromSource := dijkstra(1, source, edges, adjMatrix, target, fromDestination)
	if fromSource[destination] != int64(target) {
		return nil
	}
	return edges
}

func dijkstra(op, source int, edges [][]int, adjMatrix [][]int, target int, fromDestination []int64) []int64 {
	// 朴素的 dijistra 算法
	// adjMatrix 是一个邻接矩阵
	n := len(adjMatrix)
	dist, used := make([]int64, n), make([]bool, n)
	for i := 0; i < n; i++ {
		dist[i] = 0x3f3f3f3f3f
	}
	dist[source] = 0
	for round := 0; round < n-1; round++ {
		u := -1
		for i := 0; i < n; i++ {
			if !used[i] && (u == -1 || dist[i] < dist[u]) {
				u = i
			}
		}
		used[u] = true
		for v := 0; v < n; v++ {
			if !used[v] && adjMatrix[u][v] != -1 {
				i := adjMatrix[u][v]
				if edges[i][2] != -1 {
					dist[v] = min(dist[v], dist[u]+int64(edges[i][2]))
				} else {
					if op == 0 {
						dist[v] = min(dist[v], dist[u]+1)
					} else {
						modify := int64(target) - dist[u] - fromDestination[v]
						if modify > 0 {
							dist[v] = min(dist[v], dist[u]+modify)
							edges[i][2] = int(modify)
						} else {
							edges[i][2] = target
						}
					}
				}
			}
		}
	}
	return dist
}

func min(a, b int64) int64 {
	if a > b {
		return b
	}
	return a
}

func main() {
	assert := func(a, b [][]int) {
		if !reflect.DeepEqual(a, b) {
			fmt.Printf("a: %+v\n", a)
			fmt.Printf("b: %+v\n", b)
			panic("Not Passed")
		}
	}

	{
		n := 5
		edges := [][]int{{4, 1, -1}, {2, 0, -1}, {0, 3, -1}, {4, 3, -1}}
		source := 0
		destination := 1
		target := 5
		ans := [][]int{{4, 1, 1}, {2, 0, 1}, {0, 3, 3}, {4, 3, 1}}
		assert(modifiedGraphEdges(n, edges, source, destination, target), ans)
	}

	{
		n := 3
		edges := [][]int{{0, 1, -1}, {0, 2, 5}}
		source := 0
		destination := 2
		target := 6
		ans := [][]int(nil)
		assert(modifiedGraphEdges(n, edges, source, destination, target), ans)
	}

	{
		n := 4
		edges := [][]int{{1, 0, 4}, {1, 2, 3}, {2, 3, 5}, {0, 3, -1}}
		source := 0
		destination := 2
		target := 6
		ans := [][]int{{1, 0, 4}, {1, 2, 3}, {2, 3, 5}, {0, 3, 1}}
		assert(modifiedGraphEdges(n, edges, source, destination, target), ans)
	}
}
