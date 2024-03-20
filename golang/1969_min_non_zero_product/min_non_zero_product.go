/*
 * @Date: 2024-03-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-20
 * @FilePath: /algorithm/golang/1969_min_non_zero_product/min_non_zero_product.go
 */

// Package main ....
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minNonZeroProduct(p int) int {
	if p == 1 {
		return 1
	}
	mod := int64(1e9 + 7)
	x := fastPow(2, int64(p), mod) - 1
	y := int64(1) << uint(p-1)
	return int(fastPow(x-1, y-1, mod) * x % mod)
}

func fastPow(x, n, mod int64) int64 {
	res := int64(1)
	for n != 0 {
		if n&1 == 1 {
			res = (res * x) % mod
		}
		x = (x * x) % mod
		n >>= 1
	}
	return res
}

func main() {
	tests := []struct {
		p   int
		ans int
	}{
		{1, 1},
		{2, 6},
		{3, 1512},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minNonZeroProduct(test.p), index)
	}
}
