// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumMoney(transactions [][]int) int64 {
	var totalLose int64 = 0
	var res int = 0
	for _, t := range transactions {
		cost, cashback := t[0], t[1]
		totalLose += int64(max(cost-cashback, 0))
		res = max(res, min(cost, cashback))
	}
	return totalLose + int64(res)
}

func main() {
	tests := []struct {
		transactions [][]int
		ans          int64
	}{
		{[][]int{{2, 1}, {5, 0}, {4, 2}}, 10},
		{[][]int{{3, 0}, {0, 3}}, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimumMoney(test.transactions), index)
	}
}
