/*
 * @Date: 2024-01-18
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-18
 * @FilePath: /algorithm/golang/2171_minimum_removal/minimum_removal.go
 */

// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumRemoval(beans []int) int64 {
	n := len(beans)
	sort.Ints(beans)
	total := int64(0) // 豆子总数
	for _, bean := range beans {
		total += int64(bean)
	}
	res := total // 最少需要移除的豆子数
	for i := 0; i < n; i++ {
		res = min(res, total-int64(beans[i])*int64(n-i))
	}
	return res
}

func main() {
	tests := []struct {
		beans []int
		ans   int64
	}{
		{[]int{4, 1, 6, 5}, 4},
		{[]int{2, 10, 3, 2}, 7},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimumRemoval(test.beans), index)
	}
}
