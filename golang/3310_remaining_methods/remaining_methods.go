// Package main ...
package main

import (
	"fmt"
	"slices"
)

func remainingMethods(n int, k int, invocations [][]int) []int {
	edges := make([][]int, n)
	inDegree := make([]int, n)

	for _, inv := range invocations {
		u, v := inv[0], inv[1]
		edges[u] = append(edges[u], v)
		inDegree[v]++
	}

	queue := []int{k}
	suspicious := make([]bool, n)
	suspicious[k] = true

	for len(queue) > 0 {
		u := queue[0]
		queue = queue[1:]
		for _, v := range edges[u] {
			inDegree[v]--

			if !suspicious[v] {
				queue = append(queue, v)
				suspicious[v] = true
			}
		}
	}

	canRemoveAll := true
	remaining := []int{}

	for i := 0; i < n; i++ {
		if suspicious[i] && inDegree[i] > 0 {
			canRemoveAll = false
			break
		} else if !suspicious[i] {
			remaining = append(remaining, i)
		}
	}

	if !canRemoveAll {
		allNodes := make([]int, n)
		for i := 0; i < n; i++ {
			allNodes[i] = i
		}
		return allNodes
	}

	return remaining
}

func main() {
	tests := []struct {
		n           int
		k           int
		invocations [][]int
		ans         []int
	}{
		{4, 1, [][]int{{1, 2}, {0, 1}, {3, 2}}, []int{0, 1, 2, 3}},
		{5, 0, [][]int{{1, 2}, {0, 2}, {0, 1}, {3, 4}}, []int{3, 4}},
		{3, 2, [][]int{{1, 2}, {0, 1}, {2, 0}}, []int{}},
	}

	for _, test := range tests {
		result := remainingMethods(test.n, test.k, test.invocations)
		if slices.Equal(result, test.ans) {
			fmt.Println("test passed")
		} else {
			fmt.Println("test failed", result, test.ans)
		}
	}
}
