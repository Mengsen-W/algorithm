/*
 * @Date: 2023-10-15
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-15
 * @FilePath: /algorithm/golang/137_single_number/single_number.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func singleNumber(nums []int) int {
	freq := map[int]int{}
	for _, v := range nums {
		freq[v]++
	}
	for num, occ := range freq {
		if occ == 1 {
			return num
		}
	}
	return 0 // 不会发生，数据保证有一个元素仅出现一次
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{2, 2, 3, 2}, 3},
		{[]int{0, 1, 0, 1, 0, 1, 99}, 99},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, singleNumber(test.nums), "case:%d", index)
	}
}
