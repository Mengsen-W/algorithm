// Package main ...
package main

import (
	"container/heap"
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minCost(n int, edges [][]int) int {
	g := make([][][2]int, n)
	for _, e := range edges {
		u, v, w := e[0], e[1], e[2]
		g[u] = append(g[u], [2]int{v, w})
		g[v] = append(g[v], [2]int{u, w * 2})
	}

	inf := math.MaxInt / 2
	dist := make([]int, n)
	for i := range dist {
		dist[i] = inf
	}
	dist[0] = 0

	pq := &hp{}
	heap.Init(pq)
	heap.Push(pq, pair{0, 0})

	for pq.Len() > 0 {
		cur := heap.Pop(pq).(pair)
		d, u := cur.x, cur.i
		if d > dist[u] {
			continue
		}
		if u == n-1 {
			return d
		}
		for _, ne := range g[u] {
			v, w := ne[0], ne[1]
			if nd := d + w; nd < dist[v] {
				dist[v] = nd
				heap.Push(pq, pair{nd, v})
			}
		}
	}
	return -1
}

type (
	pair struct{ x, i int }
	hp   []pair
)

func (h hp) Len() int           { return len(h) }
func (h hp) Less(i, j int) bool { return h[i].x < h[j].x }
func (h hp) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *hp) Push(x any)        { *h = append(*h, x.(pair)) }
func (h *hp) Pop() (x any) {
	a := *h
	x = a[len(a)-1]
	*h = a[:len(a)-1]
	return
}

func main() {
	tests := []struct {
		n     int
		edges [][]int
		ans   int
	}{
		{4, [][]int{{0, 1, 3}, {3, 1, 1}, {2, 3, 4}, {0, 2, 2}}, 5},
		{4, [][]int{{0, 2, 1}, {2, 1, 1}, {1, 3, 1}, {2, 3, 3}}, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minCost(test.n, test.edges), index)
	}
}
