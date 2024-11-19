package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func shortestDistanceAfterQueries(n int, queries [][]int) []int {
	roads := make([]int, n)
	for i := 0; i < n; i++ {
		roads[i] = i + 1
	}
	var res []int
	dist := n - 1
	for _, query := range queries {
		k := roads[query[0]]
		roads[query[0]] = query[1]
		for k != -1 && k < query[1] {
			k, roads[k] = roads[k], -1
			dist--
		}
		res = append(res, dist)
	}
	return res
}

func main() {
	tests := []struct {
		n       int
		queries [][]int
		ans     []int
	}{
		{5, [][]int{{2, 4}, {0, 2}, {0, 4}}, []int{3, 2, 1}},
		{4, [][]int{{0, 3}, {0, 2}}, []int{1, 1}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, shortestDistanceAfterQueries(test.n, test.queries), index)
	}
}
