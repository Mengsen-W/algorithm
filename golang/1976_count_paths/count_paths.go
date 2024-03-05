/*
 * @Date: 2024-03-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-05
 * @FilePath: /algorithm/golang/1976_count_paths/count_paths.go
 */

// Package main ...
package main

import (
	"container/heap"
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func countPaths(n int, roads [][]int) int {
	const mod = int64(1e9 + 7)
	e := make([][]Edge, n)
	for _, road := range roads {
		x, y, t := road[0], road[1], road[2]
		e[x] = append(e[x], Edge{y, t})
		e[y] = append(e[y], Edge{x, t})
	}

	dis := make([]int64, n)
	for i := range dis {
		dis[i] = math.MaxInt64
	}
	ways := make([]int64, n)

	q := PriorityQueue{{0, 0}}
	heap.Init(&q)
	dis[0] = 0
	ways[0] = 1

	for len(q) > 0 {
		p := heap.Pop(&q).(Pair)
		t, u := p.first, p.second
		if t > dis[u] {
			continue
		}
		for _, edge := range e[u] {
			v, w := edge.to, edge.cost
			if t+int64(w) < dis[v] {
				dis[v] = t + int64(w)
				ways[v] = ways[u]
				heap.Push(&q, Pair{t + int64(w), v})
			} else if t+int64(w) == dis[v] {
				ways[v] = (ways[u] + ways[v]) % mod
			}
		}
	}
	return int(ways[n-1])
}

type Edge struct {
	to   int
	cost int
}

type Pair struct {
	first  int64
	second int
}

type PriorityQueue []Pair

func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
}

func (pq PriorityQueue) Len() int {
	return len(pq)
}

func (pq PriorityQueue) Less(i, j int) bool {
	return pq[i].first < pq[j].first
}

func (pq *PriorityQueue) Push(x any) {
	*pq = append(*pq, x.(Pair))
}

func (pq *PriorityQueue) Pop() any {
	n := len(*pq)
	x := (*pq)[n-1]
	*pq = (*pq)[:n-1]
	return x
}

func main() {
	tests := []struct {
		n     int
		roads [][]int
		ans   int
	}{
		{
			7,
			[][]int{{0, 6, 7}, {0, 1, 2}, {1, 2, 3}, {1, 3, 3}, {6, 3, 3}, {3, 5, 1}, {6, 5, 1}, {2, 5, 1}, {0, 4, 5}, {4, 6, 2}},
			4,
		},
		{2, [][]int{{1, 0, 10}}, 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countPaths(test.n, test.roads), index)
	}
}
