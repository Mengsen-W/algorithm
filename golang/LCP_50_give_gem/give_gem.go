/*
 * @Date: 2023-09-15
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-15
 * @FilePath: /algorithm/golang/LCP_50_give_gem/give_gem.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func giveGem(gem []int, operations [][]int) int {
	max := func(x int, y int) int {
		if x < y {
			return y
		}
		return x
	}

	min := func(x int, y int) int {
		if x > y {
			return y
		}
		return x
	}

	for _, operation := range operations {
		x, y := operation[0], operation[1]
		number := gem[x] / 2
		gem[x] -= number
		gem[y] += number
	}
	mn, mx := gem[0], gem[0]
	for _, number := range gem {
		mn = min(number, mn)
		mx = max(number, mx)
	}
	return mx - mn
}

func main() {
	tests := []struct {
		gem       []int
		operation [][]int
		ans       int
	}{
		{[]int{3, 1, 2}, [][]int{{0, 2}, {2, 1}, {2, 0}}, 2},
		{[]int{100, 0, 50, 100}, [][]int{{0, 2}, {0, 1}, {3, 0}, {3, 0}}, 75},
		{[]int{0, 0, 0, 0}, [][]int{{1, 2}, {3, 1}, {1, 2}}, 0},
	}

	for index, item := range tests {
		assert.Equal(&testing.T{}, item.ans, giveGem(item.gem, item.operation), index)
	}
}
