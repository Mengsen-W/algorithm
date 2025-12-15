// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func getDescentPeriods(prices []int) int64 {
	n := len(prices)
	res := int64(1) // 平滑下降阶段的总数，初值为 dp[0]
	prev := 1       // 上一个元素为结尾的平滑下降阶段的总数，初值为 dp[0]
	// 从 1 开始遍历数组，按照递推式更新 prev 以及总数 res
	for i := 1; i < n; i++ {
		if prices[i] == prices[i-1]-1 {
			prev++
		} else {
			prev = 1
		}
		res += int64(prev)
	}
	return res
}

func main() {
	tests := []struct {
		prices []int
		ans    int64
	}{
		{[]int{3, 2, 1, 4}, 7},
		{[]int{8, 6, 7, 7}, 4},
		{[]int{1}, 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, getDescentPeriods(test.prices), index)
	}
}
