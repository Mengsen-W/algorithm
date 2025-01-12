// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func largestCombination(candidates []int) int {
	// 计算从低到高第 k 个二进制位数值为 1 的元素个数
	maxlen := func(k int) int {
		res := 0
		for _, num := range candidates {
			if num&(1<<k) != 0 {
				res++
			}
		}
		return res
	}

	res := 0
	for i := 0; i < 24; i++ {
		// 遍历二进制位
		res = max(res, maxlen(i))
	}
	return res
}

func main() {
	tests := []struct {
		candidates []int
		ans        int
	}{
		{[]int{16, 17, 71, 62, 12, 24, 14}, 4},
		{[]int{8, 8}, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, largestCombination(test.candidates), test.ans, index)
	}
}
