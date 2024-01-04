/*
 * @Date: 2024-01-04
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-04
 * @FilePath: /algorithm/golang/2397_maximum_rows/maximum_rows.go
 */

// Package main ...
package main

import (
	"math/bits"
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximumRows(matrix [][]int, numSelect int) int {
	m, n := len(matrix), len(matrix[0])
	mask := make([]int, m)
	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			mask[i] += matrix[i][j] << (n - j - 1)
		}
	}
	res, limit := 0, 1<<n
	for cur := (1 << numSelect) - 1; cur < limit; {
		t := 0
		for j := 0; j < m; j++ {
			if (mask[j] & cur) == mask[j] {
				t++
			}
		}
		res = max(res, t)
		lb := cur & -cur
		r := cur + lb
		cur = ((r ^ cur) >> (bits.TrailingZeros(uint(lb)) + 2)) | r
	}
	return res
}

func main() {
	tests := []struct {
		matrix    [][]int
		numSelect int
		ans       int
	}{
		{[][]int{{0, 0, 0}, {1, 0, 1}, {0, 1, 1}, {0, 0, 1}}, 2, 3},
		{[][]int{{1}, {0}}, 1, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maximumRows(test.matrix, test.numSelect), index)
	}
}
