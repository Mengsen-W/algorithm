/*
 * @Date: 2022-04-03 23:23:18
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-13
 * @FilePath: /algorithm/golang/307_NumArray/NumArray.go
 */

package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

type NumArray struct {
	nums, tree []int
}

func Constructor(nums []int) NumArray {
	tree := make([]int, len(nums)+1)
	na := NumArray{nums, tree}
	for i, num := range nums {
		na.add(i+1, num)
	}
	return na
}

func (na *NumArray) add(index, val int) {
	for ; index < len(na.tree); index += index & -index {
		na.tree[index] += val
	}
}

func (na *NumArray) prefixSum(index int) (sum int) {
	for ; index > 0; index &= index - 1 {
		sum += na.tree[index]
	}
	return
}

func (na *NumArray) Update(index, val int) {
	na.add(index+1, val-na.nums[index])
	na.nums[index] = val
}

func (na *NumArray) SumRange(left, right int) int {
	return na.prefixSum(right+1) - na.prefixSum(left)
}

func main() {
	n := Constructor([]int{1, 3, 5})
	assert.Equal(&testing.T{}, 9, n.SumRange(0, 2))
	n.Update(1, 2)
	assert.Equal(&testing.T{}, 8, n.SumRange(0, 2))
}
