/*
 * @Date: 2024-01-25
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-25
 * @FilePath: /algorithm/golang/2859_sum_indices_with_k_set_bits/sum_indices_with_k_set_bits.go
 */

// Package main ,,,
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func sumIndicesWithKSetBits(nums []int, k int) int {
	bitCount := func(x int) int {
		x = (x & 0b0101010101) + ((x & 0b1010101010) >> 1)
		x = ((x & 0b0011001100) >> 2) + (x & 0b1100110011)
		x = (x >> 8) + ((x >> 4) & 0b1111) + (x & 0b1111)
		return x
	}
	ans := 0
	for i := 0; i < len(nums); i++ {
		if bitCount(i) == k {
			ans += nums[i]
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
		{[]int{5, 10, 1, 5, 2}, 1, 13},
		{[]int{4, 3, 2, 1}, 2, 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, sumIndicesWithKSetBits(test.nums, test.k), index)
	}
}
