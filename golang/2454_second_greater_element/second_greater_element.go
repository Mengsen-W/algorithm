/*
 * @Date: 2023-12-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-12
 * @FilePath: /algorithm/golang/2454_second_greater_element/second_greater_element.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func secondGreaterElement(nums []int) []int {
	n := len(nums)
	res := make([]int, n)
	for i := 0; i < n; i++ {
		res[i] = -1
	}
	st1, st2 := []int{}, []int{}

	for i := 0; i < n; i++ {
		v := nums[i]
		for len(st2) > 0 && nums[st2[len(st2)-1]] < v {
			res[st2[len(st2)-1]] = v
			st2 = st2[:len(st2)-1]
		}
		pos := len(st1) - 1
		for pos >= 0 && nums[st1[pos]] < v {
			pos--
		}
		st2 = append(st2, st1[pos+1:]...)
		st1 = append(st1[:pos+1], i)
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		ans  []int
	}{
		{[]int{2, 4, 0, 9, 6}, []int{9, 6, 6, -1, -1}},
		{[]int{3, 3}, []int{-1, -1}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, secondGreaterElement(test.nums), index)
	}
}
