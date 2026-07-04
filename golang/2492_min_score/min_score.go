// Package main ...
package main

import (
	"container/heap"
	"fmt"
	"math"
)

type edge struct {
	v   int
	dis int
}

type minHeap []edge

func (h minHeap) Len() int           { return len(h) }
func (h minHeap) Less(i, j int) bool { return h[i].dis < h[j].dis }
func (h minHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *minHeap) Push(x interface{}) {
	*h = append(*h, x.(edge))
}

func (h *minHeap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

func minScore(n int, roads [][]int) int {
	graph := make([][]edge, n+1)
	vis := make([]bool, n+1)

	ans := math.MaxInt32
	pq := &minHeap{}
	heap.Init(pq)

	for _, road := range roads {
		u, v, dis := road[0], road[1], road[2]
		graph[u] = append(graph[u], edge{v: v, dis: dis})
		graph[v] = append(graph[v], edge{v: u, dis: dis})

		if pq.Len() == 0 && (u == 1 || v == 1) {
			heap.Push(pq, edge{v: v, dis: dis})
		}
	}

	for pq.Len() > 0 {
		curr := heap.Pop(pq).(edge)
		u := curr.v
		dis := curr.dis

		if vis[u] {
			continue
		}

		if dis < ans {
			ans = dis
		}
		vis[u] = true

		for _, e := range graph[u] {
			if !vis[e.v] {
				heap.Push(pq, e)
			}
		}
	}

	return ans
}

func main() {
	tests := []struct {
		n     int
		roads [][]int
		ans   int
	}{
		{4, [][]int{{1, 2, 9}, {2, 3, 6}, {2, 4, 5}}, 5},
		{4, [][]int{{1, 2, 2}, {1, 3, 4}, {3, 4, 7}}, 2},
	}

	for _, tt := range tests {
		if got := minScore(tt.n, tt.roads); got != tt.ans {
			fmt.Println("got:", got, "ans:", tt.ans)
		}
	}
}
