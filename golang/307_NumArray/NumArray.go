/*
 * @Date: 2022-04-03 23:23:18
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-03 23:38:00
 * @FilePath: /algorithm/307_NumArray/NumArray.go
 */

package main

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
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	n := Constructor([]int{1, 3, 5})
	assert(n.SumRange(0, 2) == 9)
	n.Update(1, 2)
	assert(n.SumRange(0, 2) == 8)
}
