/*
 * @Date: 2021-07-12 08:06:52
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-30
 */

// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func hIndex(citations []int) int {
	n := len(citations)
	return n - sort.Search(n, func(x int) bool { return citations[x] >= n-x })
}

func main() {
	tests := []struct {
		citations []int
		ans       int
	}{
		{[]int{0, 1, 3, 5, 6}, 3},
		{[]int{1, 2, 100}, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, hIndex(test.citations), index)
	}
}
