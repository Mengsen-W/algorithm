/*
 * @Date: 2024-03-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-08
 * @FilePath: /algorithm/golang/2834_minimum_possible_sum/minimum_possible_sum.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumPossibleSum(n int, target int) int {
	const mod = 1000000007
	m := target / 2
	if n <= m {
		return ((1 + n) * n / 2) % mod
	}
	return (((1 + m) * m / 2) + ((target + target + (n - m) - 1) * (n - m) / 2)) % mod
}

func main() {
	tests := []struct {
		n      int
		target int
		ans    int
	}{
		{2, 3, 4},
		{3, 3, 8},
		{1, 1, 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimumPossibleSum(test.n, test.target), index)
	}
}
