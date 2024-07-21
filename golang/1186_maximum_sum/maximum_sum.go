// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximumSum(arr []int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	dp0, dp1, res := arr[0], 0, arr[0]
	for i := 1; i < len(arr); i++ {
		dp0, dp1 = max(dp0, 0)+arr[i], max(dp1+arr[i], dp0)
		res = max(res, max(dp0, dp1))
	}
	return res
}

func main() {
	tests := []struct {
		arr []int
		ans int
	}{
		{[]int{1, -2, 0, 3}, 4},
		{[]int{1, -2, -2, 3}, 3},
		{[]int{-1, -1, -1, -1}, -1},
	}

	for _, item := range tests {
		assert.Equal(&testing.T{}, maximumSum(item.arr), item.ans)
	}
}
