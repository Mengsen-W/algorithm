/*
 * @Date: 2023-09-27
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-27
 * @FilePath: /algorithm/golang/1333_filter_restaurants/filter_restaurants.go
 */

// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func filterRestaurants(restaurants [][]int, veganFriendly int, maxPrice int, maxDistance int) []int {
	filtered := [][]int{}
	for _, r := range restaurants {
		if r[3] <= maxPrice && r[4] <= maxDistance && (veganFriendly != 1 || r[2] != 0) {
			filtered = append(filtered, r)
		}
	}
	sort.Slice(filtered, func(i, j int) bool {
		return filtered[i][1] > filtered[j][1] || (filtered[i][1] == filtered[j][1] && filtered[i][0] > filtered[j][0])
	})
	res := []int{}
	for _, r := range filtered {
		res = append(res, r[0])
	}
	return res
}

func main() {
	tests := []struct {
		restaurants   [][]int
		veganFriendly int
		maxPrice      int
		maxDistance   int
		ans           []int
	}{
		{
			[][]int{{1, 4, 1, 40, 10}, {2, 8, 0, 50, 5}, {3, 8, 1, 30, 4}, {4, 10, 0, 10, 3}, {5, 1, 1, 15, 1}},
			1,
			50,
			10,
			[]int{3, 1, 5},
		},
		{
			[][]int{{1, 4, 1, 40, 10}, {2, 8, 0, 50, 5}, {3, 8, 1, 30, 4}, {4, 10, 0, 10, 3}, {5, 1, 1, 15, 1}},
			0,
			50,
			10,
			[]int{4, 3, 2, 1, 5},
		},
		{
			[][]int{{1, 4, 1, 40, 10}, {2, 8, 0, 50, 5}, {3, 8, 1, 30, 4}, {4, 10, 0, 10, 3}, {5, 1, 1, 15, 1}},
			0,
			30,
			3,
			[]int{4, 5},
		},
	}

	for index, item := range tests {
		assert.Equal(&testing.T{}, filterRestaurants(item.restaurants, item.veganFriendly, item.maxPrice, item.maxDistance), item.ans, index)
	}
}
