/*
 * @Date: 2022-11-26
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-26
 * @FilePath: /algorithm/882_reachable_nodes/reachable_nodes.go
 */

package main

import "container/heap"

func reachableNodes(edges [][]int, maxMoves, n int) int {
	adList := map[int][][]int{}
	for _, edge := range edges {
		u, v, nodes := edge[0], edge[1], edge[2]
		adList[u] = append(adList[u], []int{v, nodes})
		adList[v] = append(adList[v], []int{u, nodes})
	}
	used := map[int]int{}
	visited := map[int]bool{}
	reachableNodes := 0
	pq := myHeap{}
	heap.Push(&pq, []int{0, 0})

	for pq.Len() > 0 && pq[0][0] <= maxMoves {
		p := heap.Pop(&pq).([]int)
		step, u := p[0], p[1]
		if visited[u] {
			continue
		}
		visited[u] = true
		reachableNodes++
		for _, q := range adList[u] {
			v, nodes := q[0], q[1]
			if nodes+step+1 <= maxMoves && !visited[v] {
				heap.Push(&pq, []int{nodes + step + 1, v})
			}
			used[u*n+v] = min(nodes, maxMoves-step)
		}
	}

	for _, edge := range edges {
		u, v, nodes := edge[0], edge[1], edge[2]
		reachableNodes += min(nodes, used[u*n+v]+used[v*n+u])
	}
	return reachableNodes
}

func min(x, y int) int {
	if x > y {
		return y
	}
	return x
}

type myHeap [][]int

func (h myHeap) Len() int {
	return len(h)
}

func (h myHeap) Less(i, j int) bool {
	return h[i][0] < h[j][0]
}

func (h myHeap) Swap(i, j int) {
	h[i], h[j] = h[j], h[i]
}

func (h *myHeap) Push(val interface{}) {
	*h = append(*h, val.([]int))
}

func (h *myHeap) Pop() interface{} {
	hp := *h
	val := hp[len(hp)-1]
	*h = hp[:len(hp)-1]
	return val
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		edges := [][]int{{0, 1, 10}, {0, 2, 1}, {1, 2, 2}}
		maxMoves := 6
		n := 3
		ans := 13
		assert(reachableNodes(edges, maxMoves, n) == ans)
	}

	{
		edges := [][]int{{0, 1, 4}, {1, 2, 6}, {0, 2, 8}, {1, 3, 1}}
		maxMoves := 10
		n := 4
		ans := 23
		assert(reachableNodes(edges, maxMoves, n) == ans)
	}

	{
		edges := [][]int{{1, 2, 4}, {1, 4, 5}, {1, 3, 1}, {2, 3, 4}, {3, 4, 5}}
		maxMoves := 17
		n := 5
		ans := 1
		assert(reachableNodes(edges, maxMoves, n) == ans)
	}
}
