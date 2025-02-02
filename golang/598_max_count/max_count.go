// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxCount(m, n int, ops [][]int) int {
	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}

	mina, minb := m, n
	for _, op := range ops {
		mina = min(mina, op[0])
		minb = min(minb, op[1])
	}
	return mina * minb
}

func main() {
	tests := []struct {
		m   int
		n   int
		ops [][]int
		ans int
	}{
		{3, 3, [][]int{{2, 2}, {3, 3}}, 4},
		{3, 3, [][]int{{2, 2}, {3, 3}, {3, 3}, {3, 3}, {2, 2}, {3, 3}, {3, 3}, {3, 3}, {2, 2}, {3, 3}, {3, 3}, {3, 3}}, 4},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxCount(test.m, test.n, test.ops), index)
	}
}
