// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func winningPlayerCount(n int, pick [][]int) int {
	cnt := make([][]int, n)
	for i := range cnt {
		cnt[i] = make([]int, 11)
	}
	for _, p := range pick {
		cnt[p[0]][p[1]]++
	}

	ans := 0
	for i := 0; i < n; i++ {
		for j := 0; j <= 10; j++ {
			if cnt[i][j] > i {
				ans++
				break
			}
		}
	}
	return ans
}

func main() {
	tests := []struct {
		n    int
		pick [][]int
		ans  int
	}{
		{4, [][]int{{0, 0}, {1, 0}, {1, 0}, {2, 1}, {2, 1}, {2, 0}}, 2},
		{5, [][]int{{1, 1}, {1, 2}, {1, 3}, {1, 4}}, 0},
		{5, [][]int{{1, 1}, {2, 4}, {2, 4}, {2, 4}}, 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, winningPlayerCount(test.n, test.pick), index)
	}
}
