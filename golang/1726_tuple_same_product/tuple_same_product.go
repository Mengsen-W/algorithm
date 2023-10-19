/*
 * @Date: 2023-10-19
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-19
 * @FilePath: /algorithm/golang/1726_tuple_same_product/tuple_same_product.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func tupleSameProduct(nums []int) int {
	n := len(nums)
	cnt := make(map[int]int)
	for i := 0; i < n; i++ {
		for j := i + 1; j < n; j++ {
			cnt[nums[i]*nums[j]]++
		}
	}
	ans := 0
	for _, v := range cnt {
		ans += v * (v - 1) * 4
	}
	return ans
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{2, 3, 4, 6}, 8},
		{[]int{1, 2, 4, 5, 10}, 16},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, tupleSameProduct(test.nums), index)
	}
}
