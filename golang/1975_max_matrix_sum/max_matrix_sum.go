// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxMatrixSum(matrix [][]int) int64 {
	abs := func(x int) int {
		if x < 0 {
			return -x
		}
		return x
	}

	n := len(matrix)
	cnt := 0          // 负数元素的数量
	total := int64(0) // 所有元素的绝对值之和
	mn := 1 << 30     // 方阵元素的最小绝对值
	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			mn = min(mn, abs(matrix[i][j]))
			if matrix[i][j] < 0 {
				cnt++
			}
			total += int64(abs(matrix[i][j]))
		}
	}
	// 按照负数元素的数量的奇偶性讨论
	if cnt%2 == 0 {
		return total
	}
	return total - int64(2*mn)
}

func main() {
	tests := []struct {
		matrix [][]int
		ans    int64
	}{
		{[][]int{{1, -1}, {-1, 1}}, 4},
		{[][]int{{1, 2, 3}, {-1, -2, -3}, {1, 2, 3}}, 16},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxMatrixSum(test.matrix), index)
	}
}
