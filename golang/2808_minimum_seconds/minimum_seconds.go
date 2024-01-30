/*
 * @Date: 2024-01-30
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-30
 * @FilePath: /algorithm/golang/2808_minimum_seconds/minimum_seconds.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumSeconds(nums []int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}

	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}

	mp := make(map[int][]int)
	n := len(nums)
	res := n
	for i, num := range nums {
		mp[num] = append(mp[num], i)
	}
	for _, pos := range mp {
		mx := pos[0] + n - pos[len(pos)-1]
		for i := 1; i < len(pos); i++ {
			mx = max(mx, pos[i]-pos[i-1])
		}
		res = min(res, mx/2)
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 2, 1, 2}, 1},
		{[]int{2, 1, 3, 3, 2}, 2},
		{[]int{5, 5, 5, 5}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimumSeconds(test.nums), index)
	}
}
