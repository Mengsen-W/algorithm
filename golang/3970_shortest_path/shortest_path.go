// Package main ...
package main

import "container/heap"

type State struct {
	d      int64
	x, cnt int
}

type MinHeap []State

func (h MinHeap) Len() int            { return len(h) }
func (h MinHeap) Less(i, j int) bool  { return h[i].d < h[j].d }
func (h MinHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *MinHeap) Push(x interface{}) { *h = append(*h, x.(State)) }
func (h *MinHeap) Pop() interface{} {
	a := *h
	x := a[len(a)-1]
	*h = a[:len(a)-1]
	return x
}

func shortestPath(n int, edges [][]int, labels string, k int) int {
	g := make([][][2]int, n)
	for _, e := range edges {
		g[e[0]] = append(g[e[0]], [2]int{e[1], e[2]})
	}

	dis := make([][]int64, n)
	for i := range dis {
		dis[i] = make([]int64, k+1)
		for j := range dis[i] {
			dis[i][j] = 1<<31 - 1
		}
	}
	// {最短路长度, 节点编号, 最后连续相同字母个数}
	pq := &MinHeap{}

	dis[0][1] = 0
	heap.Push(pq, State{0, 0, 1})

	for pq.Len() > 0 {
		cur := heap.Pop(pq).(State)
		d, x, cnt := cur.d, cur.x, cur.cnt

		if d > dis[x][cnt] {
			continue
		}
		if x == n-1 {
			return int(d)
		}

		for _, e := range g[x] {
			y, w := e[0], e[1]
			newCnt := 1
			if labels[x] == labels[y] {
				newCnt = cnt + 1
			}
			if newCnt > k {
				continue
			}

			nd := d + int64(w)
			if nd < dis[y][newCnt] {
				dis[y][newCnt] = nd
				heap.Push(pq, State{nd, y, newCnt})
			}
		}
	}

	return -1
}

func main() {
	tests := []struct {
		n      int
		edges  [][]int
		labels string
		k      int
		ans    int
	}{
		{3, [][]int{{0, 1, 1}, {1, 2, 1}, {0, 2, 3}}, "aab", 1, 3},
		{3, [][]int{{0, 1, 1}, {1, 2, 1}, {0, 2, 3}}, "aab", 2, 2},
		{3, [][]int{{0, 1, 1}, {1, 2, 1}}, "aaa", 2, -1},
	}

	for _, test := range tests {
		if ans := shortestPath(test.n, test.edges, test.labels, test.k); ans != test.ans {
			panic("wrong answer")
		}
	}
}
