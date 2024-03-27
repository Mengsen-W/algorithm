/*
 * @Date: 2024-03-27
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-27
 * @FilePath: /algorithm/golang/2580_count_ways/count_ways.go
 */

// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func countWays(ranges [][]int) int {
	const mod = int(1e9 + 7)
	sort.Slice(ranges, func(i, j int) bool {
		return ranges[i][0] < ranges[j][0]
	})

	n := len(ranges)
	res := int64(1)
	for i := 0; i < n; {
		r := ranges[i][1]
		j := i + 1
		for j < n && ranges[j][0] <= r {
			r = max(r, ranges[j][1])
			j++
		}
		res = (res * 2) % int64(mod)
		i = j
	}
	return int(res)
}

func main() {
	tests := []struct {
		ranges [][]int
		ans    int
	}{
		{[][]int{{6, 10}, {5, 15}}, 2},
		{[][]int{{1, 3}, {10, 20}, {2, 5}, {4, 8}}, 4},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countWays(test.ranges), index)
	}
}
