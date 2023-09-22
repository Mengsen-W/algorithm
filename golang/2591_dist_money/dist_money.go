/*
 * @Date: 2023-09-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-22
 * @FilePath: /algorithm/golang/2591_dist_money/dist_money.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func distMoney(money int, children int) int {
	min := func(x int, y int) int {
		if x > y {
			return y
		}
		return x
	}

	if money < children {
		return -1
	}
	money -= children
	cnt := min(money/7, children)
	money -= cnt * 7
	children -= cnt
	if (children == 0 && money > 0) || (children == 1 && money == 3) {
		cnt -= 1
	}
	return cnt
}

func main() {
	tests := []struct {
		money    int
		children int
		ans      int
	}{
		{20, 3, 1},
		{16, 2, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, distMoney(test.money, test.children), index)
	}
}
