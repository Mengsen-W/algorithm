/*
 * @Date: 2023-10-02
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-02
 * @FilePath: /algorithm/golang/122_max_profit/max_profit.go
 */

package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxProfit(prices []int) (ans int) {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	for i := 1; i < len(prices); i++ {
		ans += max(0, prices[i]-prices[i-1])
	}
	return
}

func main() {
	tests := []struct {
		prices []int
		ans    int
	}{
		{[]int{7, 1, 5, 3, 6, 4}, 7},
		{[]int{1, 2, 3, 4, 5}, 4},
		{[]int{7, 6, 4, 3, 1}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxProfit(test.prices), index)
	}
}
