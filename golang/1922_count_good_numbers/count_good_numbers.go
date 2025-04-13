// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countGoodNumbers(n int64) int {
	mod := int64(1000000007)

	// 快速幂求出 x^y % mod
	quickmul := func(x, y int64) int64 {
		ret := int64(1)
		mul := x
		for y > 0 {
			if y%2 == 1 {
				ret = ret * mul % mod
			}
			mul = mul * mul % mod
			y /= 2
		}
		return ret
	}

	return int(quickmul(5, (n+1)/2) * quickmul(4, n/2) % mod)
}

func main() {
	tests := []struct {
		n   int64
		ans int
	}{
		{1, 5},
		{4, 400},
		{50, 564908303},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countGoodNumbers(test.n), index)
	}
}
