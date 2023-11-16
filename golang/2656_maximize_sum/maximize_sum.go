/*
 * @Date: 2023-11-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-16
 * @FilePath: /algorithm/golang/2656_maximize_sum/maximize_sum.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximizeSum(nums []int, k int) int {
	m := nums[0]
	for _, num := range nums {
		if num > m {
			m = num
		}
	}
	return (2*m + k - 1) * k / 2
}

func main() {
	tests := []struct {
		nums []int
		k    int
		ans  int
	}{
		{[]int{1, 2, 3, 4, 5}, 3, 18},
		{[]int{5, 5, 5}, 2, 11},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maximizeSum(test.nums, test.k), index)
	}
}
