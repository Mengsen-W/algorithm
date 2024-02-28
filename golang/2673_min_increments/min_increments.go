/*
 * @Date: 2024-02-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-28
 * @FilePath: /algorithm/golang/2673_min_increments/min_increments.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minIncrements(n int, cost []int) int {
	abs := func(x int) int {
		if x < 0 {
			return -x
		}
		return x
	}

	ans := 0
	for i := n - 2; i > 0; i -= 2 {
		ans += abs(cost[i] - cost[i+1])
		// 叶节点 i 和 i+1 的双亲节点下标为 i/2（整数除法）
		cost[i/2] = cost[i/2] + max(cost[i], cost[i+1])
	}
	return ans
}

func main() {
	tests := []struct {
		n    int
		cost []int
		ans  int
	}{
		{7, []int{1, 5, 2, 2, 3, 3, 1}, 6},
		{3, []int{5, 3, 3}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minIncrements(test.n, test.cost), index)
	}
}
