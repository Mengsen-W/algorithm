// Package main ...
package main

import (
	"math"
	"slices"
	"testing"

	"github.com/stretchr/testify/assert"
)

func dfs(cards []float64) bool {
	const eps = 1e-9
	n := len(cards)
	if n == 1 {
		return math.Abs(cards[0]-24) < eps
	}

	// 选两张牌 x=cards[i] 和 y=cards[j]
	for i, x := range cards {
		for j := i + 1; j < n; j++ {
			y := cards[j]

			// 六种情况：加减乘除，其中减和除都有两种不同的顺序
			candidates := []float64{x + y, x - y, y - x, x * y}
			if math.Abs(y) > eps { // 保证分母不为 0
				candidates = append(candidates, x/y)
			}
			if math.Abs(x) > eps { // 保证分母不为 0
				candidates = append(candidates, y/x)
			}

			newCards := append(slices.Clone(cards[:j]), cards[j+1:]...) // 删除 j
			for _, res := range candidates {
				newCards[i] = res // 覆盖 i
				if dfs(newCards) {
					return true
				}
			}
		}
	}
	return false
}

func judgePoint24(cards []int) bool {
	a := make([]float64, len(cards))
	for i, x := range cards {
		a[i] = float64(x)
	}
	return dfs(a)
}

func main() {
	tests := []struct {
		cards  []int
		expect bool
	}{
		{[]int{4, 1, 8, 7}, true},
		{[]int{1, 2, 1, 2}, false},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.expect, judgePoint24(test.cards), index)
	}
}
