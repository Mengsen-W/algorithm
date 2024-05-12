/*
 * @Date: 2024-05-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-12
 * @FilePath: /algorithm/golang/1553_min_days/min_days.go
 */

// Package main ...
package main

import (
	"container/heap"
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minDays(n int) int {
	q := &PriorityQueue{Node{0 + getHeuristicValue(n), 0, n}}
	visited := make(map[int]bool)
	ans := 0
	for {
		p := heap.Pop(q).(Node)
		days, rest := p.days, p.rest
		if visited[rest] {
			continue
		}
		visited[rest] = true
		if rest == 1 {
			ans = days + 1
			break
		}
		heap.Push(q, Node{days + rest%2 + 1 + getHeuristicValue(rest/2), days + rest%2 + 1, rest / 2})
		heap.Push(q, Node{days + rest%3 + 1 + getHeuristicValue(rest/3), days + rest%3 + 1, rest / 3})
	}
	return ans
}

func getHeuristicValue(rest int) int {
	if rest == 0 {
		return 0
	} else {
		return int(math.Log(float64(rest))/math.Log(3.0)) + 1
	}
}

type Node struct {
	cost, days, rest int
}

type PriorityQueue []Node

func (pq PriorityQueue) Len() int {
	return len(pq)
}

func (pq PriorityQueue) Less(i, j int) bool {
	return pq[i].cost < pq[j].cost
}

func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
}

func (pq *PriorityQueue) Push(x interface{}) {
	*pq = append(*pq, x.(Node))
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
		n   int
		ans int
	}{
		{10, 4},
		{6, 3},
		{1, 1},
		{56, 6},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minDays(test.n), index)
	}
}
