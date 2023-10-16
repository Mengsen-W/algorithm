/*
 * @Date: 2021-10-30 01:13:14
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-16
 */

package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func singleNumber(nums []int) []int {
	xorSum := 0
	for _, num := range nums {
		xorSum ^= num
	}
	lsb := xorSum & -xorSum
	type1, type2 := 0, 0
	for _, num := range nums {
		if num&lsb > 0 {
			type1 ^= num
		} else {
			type2 ^= num
		}
	}
	return []int{type1, type2}
}

func main() {
	tests := []struct {
		nums []int
		ans  []int
	}{
		{[]int{1, 2, 1, 3, 2, 5}, []int{3, 5}},
		{[]int{-1, 0}, []int{-1, 0}},
		{[]int{0, 1}, []int{1, 0}},
	}
	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, singleNumber(test.nums), index)
	}
}
