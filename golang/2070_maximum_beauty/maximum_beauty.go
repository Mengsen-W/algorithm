// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximumBeauty(items [][]int, queries []int) []int {
	// 将物品按价格升序排序
	sort.Slice(items, func(i, j int) bool {
		return items[i][0] < items[j][0]
	})
	n := len(items)
	// 按定义修改美丽值
	for i := 1; i < n; i++ {
		if items[i][1] < items[i-1][1] {
			items[i][1] = items[i-1][1]
		}
	}
	// 二分查找处理查询
	res := make([]int, len(queries))
	for i, q := range queries {
		res[i] = query(items, q)
	}
	return res
}

func query(items [][]int, q int) int {
	l, r := 0, len(items)
	for l < r {
		mid := l + (r-l)/2
		if items[mid][0] > q {
			r = mid
		} else {
			l = mid + 1
		}
	}
	if l == 0 {
		// 此时所有物品价格均大于查询价格
		return 0
	} else {
		// 返回小于等于查询价格的物品的最大美丽值
		return items[l-1][1]
	}
}

func main() {
	tests := []struct {
		items   [][]int
		queries []int
		ans     []int
	}{
		{[][]int{{1, 2}, {3, 2}, {2, 4}, {5, 6}, {3, 5}}, []int{1, 2, 3, 4, 5, 6}, []int{2, 4, 5, 5, 6, 6}},
		{[][]int{{1, 2}, {1, 2}, {1, 3}, {1, 4}}, []int{1}, []int{4}},
		{[][]int{{10, 1000}}, []int{5}, []int{0}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maximumBeauty(test.items, test.queries), index)
	}
}
