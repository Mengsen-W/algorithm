/*
 * @Date: 2024-03-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-06
 * @FilePath: /algorithm/golang/2917_find_k_or/find_k_or.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func findKOr(nums []int, k int) int {
	ans := 0
	for i := 0; i < 31; i++ {
		cnt := 0
		for _, num := range nums {
			if (num>>i)&1 == 1 {
				cnt++
			}
		}
		if cnt >= k {
			ans |= 1 << i
		}
	}
	return ans
}

func main() {
	tests := []struct {
		nums []int
		k    int
		ans  int
	}{
		{[]int{7, 12, 9, 8, 9, 15}, 4, 9},
		{[]int{2, 12, 1, 11, 4, 5}, 6, 0},
		{[]int{10, 8, 5, 9, 11, 6, 8}, 1, 15},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, findKOr(test.nums, test.k), index)
	}
}
