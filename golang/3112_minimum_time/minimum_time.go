// Package main ...
package main

import (
	"container/heap"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumTime(n int, edges [][]int, disappear []int) []int {
	adj := make([][]struct{ v, length int }, n)
	for _, edge := range edges {
		u, v, length := edge[0], edge[1], edge[2]
		adj[u] = append(adj[u], struct{ v, length int }{v, length})
		adj[v] = append(adj[v], struct{ v, length int }{u, length})
	}
	pq := &PriorityQueue{}
	heap.Init(pq)
	heap.Push(pq, Item{0, 0})
	answer := make([]int, n)
	for i := range answer {
		answer[i] = -1
	}
	answer[0] = 0
	for pq.Len() > 0 {
		item := heap.Pop(pq).(Item)
		t, u := item.priority, item.value
		if t != answer[u] {
			continue
		}
		for _, edge := range adj[u] {
			v, length := edge.v, edge.length
			if t+length < disappear[v] && (answer[v] == -1 || t+length < answer[v]) {
				heap.Push(pq, Item{t + length, v})
				answer[v] = t + length
			}
		}
	}
	return answer
}

type Item struct {
	priority, value int
}

type PriorityQueue []Item

func (pq PriorityQueue) Len() int { return len(pq) }

func (pq PriorityQueue) Less(i, j int) bool {
	return pq[i].priority < pq[j].priority
}

func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
}

func (pq *PriorityQueue) Push(x interface{}) {
	*pq = append(*pq, x.(Item))
}

func (pq *PriorityQueue) Pop() interface{} {
	old := *pq
	n := len(old)
	item := old[n-1]
	*pq = old[0 : n-1]
	return item
}

func main() {
	tests := []struct {
		n         int
		edges     [][]int
		disappear []int
		ans       []int
	}{
		{3, [][]int{{0, 1, 2}, {1, 2, 1}, {0, 2, 4}}, []int{1, 1, 5}, []int{0, -1, 4}},
		{3, [][]int{{0, 1, 2}, {1, 2, 1}, {0, 2, 4}}, []int{1, 3, 5}, []int{0, 2, 3}},
		{2, [][]int{{0, 1, 1}}, []int{1, 1}, []int{0, -1}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimumTime(test.n, test.edges, test.disappear), index)
	}
}
