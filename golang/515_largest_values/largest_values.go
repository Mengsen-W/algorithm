/*
 * @Date: 2022-06-24
 * @LastEditors: Mengsen Wang mengsen_wang@163.com
 * @LastEditTime: 2022-06-24
 * @FilePath: /algorithm/515_largest_values/largest_values.go
 */

package main

import (
	"math"
	"reflect"
)

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func largestValues(root *TreeNode) (ans []int) {
	if root == nil {
		return
	}
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}
	q := []*TreeNode{root}
	for len(q) > 0 {
		maxVal := math.MinInt32
		tmp := q
		q = nil
		for _, node := range tmp {
			maxVal = max(maxVal, node.Val)
			if node.Left != nil {
				q = append(q, node.Left)
			}
			if node.Right != nil {
				q = append(q, node.Right)
			}
		}
		ans = append(ans, maxVal)
	}
	return
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	{
		root := &TreeNode{1, &TreeNode{3, &TreeNode{5, nil, nil}, &TreeNode{3, nil, nil}}, &TreeNode{2, nil, &TreeNode{9, nil, nil}}}
		ans := []int{1, 3, 9}
		assert(largestValues(root), ans)
	}

	{
		root := &TreeNode{1, &TreeNode{2, nil, nil}, &TreeNode{3, nil, nil}}
		ans := []int{1, 3}
		assert(largestValues(root), ans)
	}
}
