/*
 * @Date: 2023-10-14
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-14
 * @FilePath: /algorithm/golang/136_single_number/single_number.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func singleNumber(nums []int) int {
	single := 0
	for _, num := range nums {
		single ^= num
	}
	return single
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{2, 2, 1}, 1},
		{[]int{4, 1, 2, 1, 2}, 4},
		{[]int{1}, 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, singleNumber(test.nums), index)
	}
}
