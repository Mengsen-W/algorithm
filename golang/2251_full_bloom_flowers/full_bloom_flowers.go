/*
 * @Date: 2023-09-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-28
 * @FilePath: /algorithm/golang/2251_full_bloom_flowers/full_bloom_flowers.go
 */

// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func fullBloomFlowers(flowers [][]int, people []int) []int {
	n := len(flowers)
	starts := make([]int, n)
	ends := make([]int, n)
	for i, flower := range flowers {
		starts[i] = flower[0]
		ends[i] = flower[1]
	}
	sort.Ints(starts)
	sort.Ints(ends)
	m := len(people)
	ans := make([]int, m)
	for i, p := range people {
		x := sort.SearchInts(starts, p+1)
		y := sort.SearchInts(ends, p)
		ans[i] = x - y
	}
	return ans
}

func main() {
	tests := []struct {
		flowers [][]int
		people  []int
		ans     []int
	}{
		{[][]int{{1, 6}, {3, 7}, {9, 12}, {4, 13}}, []int{2, 3, 7, 11}, []int{1, 2, 2, 2}},
		{[][]int{{1, 10}, {3, 3}}, []int{3, 3, 2}, []int{2, 2, 1}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, fullBloomFlowers(test.flowers, test.people), index)
	}
}
