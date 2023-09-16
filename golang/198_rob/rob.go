/*
 * @Date: 2023-09-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-16
 * @FilePath: /algorithm/golang/198_rob/rob.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func rob(nums []int) int {
	max := func(a, b int) int {
		if a < b {
			return b
		}
		return a
	}
	prev, curr := 0, 0
	for _, v := range nums {
		prev, curr = curr, max(prev+v, curr)
	}
	return curr
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 2, 3, 1}, 4},
		{[]int{2, 7, 9, 3, 1}, 12},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, rob(test.nums), index)
	}
}
