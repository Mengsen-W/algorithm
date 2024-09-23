// Package main ..
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxScoreSightseeingPair(values []int) int {
	max := func(x, y int) int {
		if x > y {
			return x
		}
		return y
	}
	ans, mx := 0, values[0]+0
	for j := 1; j < len(values); j++ {
		ans = max(ans, mx+values[j]-j)
		// 边遍历边维护
		mx = max(mx, values[j]+j)
	}
	return ans
}

func main() {
	tests := []struct {
		values []int
		ans    int
	}{
		{[]int{8, 1, 5, 2, 6}, 11},
		{[]int{1, 2}, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxScoreSightseeingPair(test.values), index)
	}
}
