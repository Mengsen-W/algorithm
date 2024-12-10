// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func knightDialer(n int) int {
	const mod = 1_000_000_007
	moves := [][]int{{4, 6}, {6, 8}, {7, 9}, {4, 8}, {3, 9, 0}, {}, {1, 7, 0}, {2, 6}, {1, 3}, {2, 4}}
	d := [2][10]int{}
	for i := 0; i < 10; i++ {
		d[1][i] = 1
	}
	for i := 2; i <= n; i++ {
		x := i & 1
		for j := 0; j < 10; j++ {
			d[x][j] = 0
			for _, k := range moves[j] {
				d[x][j] = (d[x][j] + d[x^1][k]) % mod
			}
		}
	}
	res := 0
	for _, x := range d[n%2] {
		res = (res + x) % mod
	}
	return res
}

func main() {
	tests := []struct {
		n   int
		ans int
	}{
		{1, 10},
		{2, 20},
		{3131, 136006598},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, knightDialer(test.n), index)
	}
}
