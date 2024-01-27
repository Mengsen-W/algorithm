/*
 * @Date: 2024-01-27
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-27
 * @FilePath: /algorithm/golang/2861_max_number_of_alloys/max_number_of_alloys.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxNumberOfAlloys(n int, k int, budget int, composition [][]int, stock []int, cost []int) int {
	left, right, ans := 1, int(2e8), 0
	for left <= right {
		mid := (left + right) / 2
		var valid bool
		for i := 0; i < k; i++ {
			var spend int64
			for j := 0; j < n; j++ {
				spend += max(int64(composition[i][j])*int64(mid)-int64(stock[j]), int64(0)) * int64(cost[j])
			}
			if spend <= int64(budget) {
				valid = true
				break
			}
		}
		if valid {
			ans, left = mid, mid+1
		} else {
			right = mid - 1
		}
	}
	return ans
}

func main() {
	tests := []struct {
		n           int
		k           int
		budget      int
		composition [][]int
		stock       []int
		cost        []int
		ans         int
	}{
		{3, 2, 15, [][]int{{1, 1, 1}, {1, 1, 10}}, []int{0, 0, 0}, []int{1, 2, 3}, 2},
		{3, 2, 15, [][]int{{1, 1, 1}, {1, 1, 10}}, []int{0, 0, 100}, []int{1, 2, 3}, 5},
		{2, 3, 10, [][]int{{2, 1}, {1, 2}, {1, 1}}, []int{1, 1}, []int{5, 5}, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxNumberOfAlloys(test.n, test.k, test.budget, test.composition, test.stock, test.cost), index)
	}
}
