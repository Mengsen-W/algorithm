/*
 * @Date: 2024-05-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-26
 * @FilePath: /algorithm/golang/1673_most_competitive/most_competitive.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func mostCompetitive(nums []int, k int) []int {
	res := make([]int, 0, len(nums))
	for i, x := range nums {
		for len(res) > 0 && len(nums)-i+len(res) > k && res[len(res)-1] > x {
			res = res[:len(res)-1]
		}
		res = append(res, x)
	}
	return res[:k]
}

func main() {
	tests := []struct {
		nums []int
		k    int
		ans  []int
	}{
		{[]int{3, 5, 2, 6}, 2, []int{2, 6}},
		{[]int{2, 4, 3, 3, 5, 4, 9, 6}, 4, []int{2, 3, 3, 4}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, mostCompetitive(test.nums, test.k), index)
	}
}
