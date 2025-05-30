// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func closestMeetingNode(edges []int, node1 int, node2 int) int {
	n := len(edges)
	get := func(node int) []int {
		n := len(edges)
		dist := make([]int, n)
		for i := range dist {
			dist[i] = -1
		}
		distance := 0
		for node != -1 && dist[node] == -1 {
			dist[node] = distance
			distance++
			node = edges[node]
		}
		return dist
	}

	d1 := get(node1)
	d2 := get(node2)
	res := -1
	for i := 0; i < n; i++ {
		if d1[i] != -1 && d2[i] != -1 {
			if res == -1 || max(d1[res], d2[res]) > max(d1[i], d2[i]) {
				res = i
			}
		}
	}
	return res
}

func main() {
	tests := []struct {
		edges []int
		node1 int
		node2 int
		ans   int
	}{
		{[]int{2, 2, 3, -1}, 0, 1, 2},
		{[]int{1, 2, -1}, 0, 2, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, closestMeetingNode(test.edges, test.node1, test.node2), "test %d", index)
	}
}
