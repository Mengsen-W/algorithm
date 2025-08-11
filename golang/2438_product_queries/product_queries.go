// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func productQueries(n int, queries [][]int) []int {
	const mod = 1000000007
	var bins []int
	rep := 1
	for n > 0 {
		if n%2 == 1 {
			bins = append(bins, rep)
		}
		n /= 2
		rep *= 2
	}

	m := len(bins)
	results := make([][]int, m)
	for i := range results {
		results[i] = make([]int, m)
		cur := 1
		for j := i; j < m; j++ {
			cur = (cur * bins[j]) % mod
			results[i][j] = cur
		}
	}

	ans := make([]int, len(queries))
	for i, query := range queries {
		ans[i] = results[query[0]][query[1]]
	}
	return ans
}

func main() {
	tests := []struct {
		n        int
		queries  [][]int
		expected []int
	}{
		{15, [][]int{{0, 1}, {2, 2}, {0, 3}}, []int{2, 4, 64}},
		{2, [][]int{{0, 0}}, []int{2}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.expected, productQueries(test.n, test.queries), index)
	}
}
