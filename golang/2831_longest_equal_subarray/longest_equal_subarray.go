/*
 * @Date: 2024-05-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-23
 * @FilePath: /algorithm/golang/2831_longest_equal_subarray/longest_equal_subarray.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func longestEqualSubarray(nums []int, k int) int {
	pos := make(map[int][]int)
	for i, num := range nums {
		pos[num] = append(pos[num], i)
	}
	ans := 0
	for _, vec := range pos {
		j := 0
		for i := 0; i < len(vec); i++ {
			/* 缩小窗口，直到不同元素数量小于等于 k */
			for vec[i]-vec[j]-(i-j) > k {
				j++
			}
			ans = max(ans, i-j+1)
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
		{[]int{1, 3, 2, 3, 1, 3}, 3, 3},
		{[]int{1, 1, 2, 2, 1, 1}, 2, 4},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, longestEqualSubarray(test.nums, test.k), index)
	}
}
