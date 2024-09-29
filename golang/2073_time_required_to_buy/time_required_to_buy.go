// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func timeRequiredToBuy(tickets []int, k int) int {
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}
	n := len(tickets)
	res := 0
	for i := 0; i < n; i++ {
		// 遍历计算每个人所需时间
		if i <= k {
			res += min(tickets[i], tickets[k])
		} else {
			res += min(tickets[i], tickets[k]-1)
		}
	}
	return res
}

func main() {
	tests := []struct {
		tickets []int
		k       int
		ans     int
	}{
		{[]int{2, 3, 2}, 2, 6},
		{[]int{5, 1, 1, 1}, 0, 8},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, timeRequiredToBuy(test.tickets, test.k), index)
	}
}
