/*
 * @Date: 2024-03-30
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-30
 * @FilePath: /algorithm/golang/2952_minimum_added_coins/minimum_added_coins.go
 */

// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumAddedCoins(coins []int, target int) (ans int) {
	sort.Ints(coins)
	for i, x := 0, 1; x <= target; {
		if i < len(coins) && coins[i] <= x {
			x += coins[i]
			i++
		} else {
			x *= 2
			ans++
		}
	}
	return
}

func main() {
	tests := []struct {
		coins  []int
		target int
		ans    int
	}{
		{[]int{1, 4, 10}, 19, 2},
		{[]int{1, 4, 10, 5, 7, 19}, 19, 1},
		{[]int{1, 1, 1}, 20, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimumAddedCoins(test.coins, test.target), index)
	}
}
