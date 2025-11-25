// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func smallestRepunitDivByK(k int) int {
	// 如果 k 是偶数或者以 5 结尾，那么一定不存在这样的正整数，直接返回 -1。
	if k%2 == 0 || k%5 == 0 {
		return -1
	}
	// cur 代表当前的余数，初始值为 0，res 代表数字 1 的个数，初始值为 1。
	cur, res := 0, 1

	// 不断计算下一个数字的余数，直到找到一个可行的解或者发现不存在可行的解。
	for {
		cur = (10*cur + 1) % k
		if cur == 0 {
			return res
		}
		res++
	}
}

func main() {
	tests := []struct {
		k   int
		ans int
	}{
		{1, 1},
		{2, -1},
		{3, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, smallestRepunitDivByK(test.k), index)
	}
}
