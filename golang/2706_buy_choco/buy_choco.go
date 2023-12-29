/*
 * @Date: 2023-12-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-29
 * @FilePath: /algorithm/golang/2706_buy_choco/buy_choco.go
 */

// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func buyChoco(prices []int, money int) int {
	fi, se := math.MaxInt64, math.MaxInt64
	for _, price := range prices {
		if price < fi {
			se, fi = fi, price
		} else if price < se {
			se = price
		}
	}
	if money < fi+se {
		return money
	}
	return money - fi - se
}

func main() {
	tests := []struct {
		prices []int
		money  int
		ans    int
	}{
		{[]int{1, 2, 2}, 3, 0},
		{[]int{3, 2, 3}, 3, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, buyChoco(test.prices, test.money), index)
	}
}
