/*
 * @Date: 2023-09-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-01
 * @FilePath: /algorithm/golang/2240_ways_to_buy_pens_pencils/ways_to_buy_pens_pencils.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func waysToBuyPensPencils(total int, cost1 int, cost2 int) int64 {
	if cost1 < cost2 {
		cost1, cost2 = cost2, cost1
	}
	ans := int64(0)
	for s := 0; s <= total; s += cost1 {
		ans += int64(total-s)/int64(cost2) + 1
	}
	return ans
}

func main() {
	tests := []struct {
		total int
		cost1 int
		cost2 int
		ans   int64
	}{
		{20, 10, 5, 9},
		{5, 10, 10, 1},
	}

	for _, item := range tests {
		assert.Equal(&testing.T{}, item.ans, waysToBuyPensPencils(item.total, item.cost1, item.cost2))
	}
}
