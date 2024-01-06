/*
 * @Date: 2024-01-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-05
 * @FilePath: /algorithm/golang/1944_can_see_persons_count/can_see_persons_count.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func canSeePersonsCount(heights []int) []int {
	n := len(heights)
	stack := make([]int, 0)
	res := make([]int, n)

	for i := n - 1; i >= 0; i-- {
		h := heights[i]
		for len(stack) > 0 && stack[len(stack)-1] <= h {
			stack = stack[:len(stack)-1]
			res[i]++
		}
		if len(stack) > 0 {
			res[i]++
		}
		stack = append(stack, h)
	}
	return res
}

func main() {
	tests := []struct {
		heights []int
		ans     []int
	}{
		{[]int{10, 6, 8, 5, 11, 9}, []int{3, 1, 2, 1, 1, 0}},
		{[]int{5, 1, 2, 3, 10}, []int{4, 1, 1, 1, 0}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, canSeePersonsCount(test.heights), index)
	}
}
