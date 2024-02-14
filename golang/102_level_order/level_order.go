/*
 * @Date: 2024-02-14
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-14
 * @FilePath: /algorithm/golang/102_level_order/level_order.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func levelOrder(root *TreeNode) [][]int {
	ret := [][]int{}
	if root == nil {
		return ret
	}
	q := []*TreeNode{root}
	for i := 0; len(q) > 0; i++ {
		ret = append(ret, []int{})
		p := []*TreeNode{}
		for j := 0; j < len(q); j++ {
			node := q[j]
			ret[i] = append(ret[i], node.Val)
			if node.Left != nil {
				p = append(p, node.Left)
			}
			if node.Right != nil {
				p = append(p, node.Right)
			}
		}
		q = p
	}
	return ret
}

func main() {
	tests := []struct {
		root *TreeNode
		ans  [][]int
	}{
		{&TreeNode{3, &TreeNode{9, nil, nil}, &TreeNode{20, &TreeNode{15, nil, nil}, &TreeNode{7, nil, nil}}}, [][]int{{3}, {9, 20}, {15, 7}}},
		{&TreeNode{1, nil, nil}, [][]int{{1}}},
		{nil, [][]int{}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, levelOrder(test.root), index)
	}
}
