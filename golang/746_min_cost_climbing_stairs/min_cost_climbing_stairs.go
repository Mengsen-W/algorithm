/*
 * @Date: 2023-12-17
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-17
 * @FilePath: /algorithm/golang/746_min_cost_climbing_stairs/min_cost_climbing_stairs.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minCostClimbingStairs(cost []int) int {
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}
	n := len(cost)
	pre, cur := 0, 0
	for i := 2; i <= n; i++ {
		pre, cur = cur, min(cur+cost[i-1], pre+cost[i-2])
	}
	return cur
}

func main() {
	tests := []struct {
		cost []int
		ans  int
	}{
		{[]int{10, 15, 20}, 15},
		{[]int{1, 100, 1, 1, 1, 100, 1, 1, 100, 1}, 6},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minCostClimbingStairs(test.cost), index)
	}
}
