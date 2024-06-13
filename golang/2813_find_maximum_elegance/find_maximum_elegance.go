// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func findMaximumElegance(items [][]int, k int) int64 {
	sort.Slice(items, func(i, j int) bool {
		return items[i][0] > items[j][0]
	})
	categorySet := map[int]bool{}
	var res, profit int64
	var st []int
	for i, item := range items {
		if i < k {
			profit += int64(item[0])
			if categorySet[item[1]] {
				st = append(st, item[0])
			} else {
				categorySet[item[1]] = true
			}
		} else if !categorySet[item[1]] && len(st) > 0 {
			profit += int64(item[0] - st[len(st)-1])
			st = st[:len(st)-1]
			categorySet[item[1]] = true
		}
		res = max(res, profit+int64(len(categorySet)*len(categorySet)))
	}
	return res
}

func main() {
	tests := []struct {
		items [][]int
		k     int
		ans   int64
	}{
		{[][]int{{3, 2}, {5, 1}, {10, 1}}, 2, 17},
		{[][]int{{3, 1}, {3, 1}, {2, 2}, {5, 3}}, 3, 19},
		{[][]int{{1, 1}, {2, 1}, {3, 1}}, 3, 7},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, findMaximumElegance(test.items, test.k), index)
	}
}
