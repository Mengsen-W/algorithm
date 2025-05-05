// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

const mod int = 1e9 + 7

type matrix [4][4]int

func (a matrix) mul(b matrix) matrix {
	c := matrix{}
	for i, row := range a {
		for j := range b[0] {
			for k, v := range row {
				c[i][j] = (c[i][j] + v*b[k][j]) % mod
			}
		}
	}
	return c
}

func (a matrix) pow(n int) matrix {
	res := matrix{}
	for i := range res {
		res[i][i] = 1
	}
	for ; n > 0; n >>= 1 {
		if n&1 > 0 {
			res = res.mul(a)
		}
		a = a.mul(a)
	}
	return res
}

func numTilings(n int) int {
	m := matrix{
		{0, 0, 0, 1},
		{1, 0, 1, 0},
		{1, 1, 0, 0},
		{1, 1, 1, 1},
	}
	return m.pow(n)[3][3]
}

func main() {
	tests := []struct {
		n   int
		ans int
	}{
		{1, 1},
		{3, 5},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, numTilings(test.n), index)
	}
}
