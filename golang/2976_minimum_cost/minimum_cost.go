// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumCost(source string, target string, original []byte, changed []byte, cost []int) int64 {
	const INF = int(^uint(0)>>1) / 2

	G := make([][]int, 26)
	for i := range G {
		G[i] = make([]int, 26)
		for j := range G[i] {
			G[i][j] = INF
		}
		G[i][i] = 0
	}

	m := len(original)
	for i := 0; i < m; i++ {
		idx := int(original[i] - 'a')
		idy := int(changed[i] - 'a')
		if cost[i] < G[idx][idy] {
			G[idx][idy] = cost[i]
		}
	}

	for k := 0; k < 26; k++ {
		for i := 0; i < 26; i++ {
			for j := 0; j < 26; j++ {
				if G[i][k] != INF && G[k][j] != INF {
					G[i][j] = min(G[i][j], G[i][k]+G[k][j])
				}
			}
		}
	}

	n := len(source)
	var ans int64 = 0
	for i := 0; i < n; i++ {
		idx := int(source[i] - 'a')
		idy := int(target[i] - 'a')
		if G[idx][idy] == INF {
			return -1
		}
		ans += int64(G[idx][idy])
	}

	return ans
}

func main() {
	tests := []struct {
		source   string
		target   string
		original []byte
		changed  []byte
		cost     []int
		ans      int64
	}{
		{"abcd", "acbe", []byte{'a', 'b', 'c', 'c', 'e', 'd'}, []byte{'b', 'c', 'b', 'e', 'b', 'e'}, []int{2, 5, 5, 1, 2, 20}, 28},
		{"aaaa", "bbbb", []byte{'a', 'c'}, []byte{'c', 'b'}, []int{1, 2}, 12},
		{"abcd", "acbe", []byte{'a'}, []byte{'e'}, []int{10000}, -1},
	}

	for _, test := range tests {
		assert.Equal(&testing.T{}, minimumCost(test.source, test.target, test.original, test.changed, test.cost), test.ans)
	}
}
