/*
 * @Date: 2024-03-18
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-18
 * @FilePath: /algorithm/golang/303_NumArray/NumArray.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

type NumArray struct {
	sums []int
}

func Constructor(nums []int) NumArray {
	sums := make([]int, len(nums)+1)
	for i, v := range nums {
		sums[i+1] = sums[i] + v
	}
	return NumArray{sums}
}

func (na *NumArray) SumRange(i, j int) int {
	return na.sums[j+1] - na.sums[i]
}

func main() {
	numArray := Constructor([]int{-2, 0, 3, -5, 2, -1})
	assert.Equal(&testing.T{}, numArray.SumRange(0, 2), 1)
	assert.Equal(&testing.T{}, numArray.SumRange(2, 5), -1)
	assert.Equal(&testing.T{}, numArray.SumRange(0, 5), -3)
}
