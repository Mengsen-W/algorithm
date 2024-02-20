/*
 * @Date: 2024-02-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-20
 * @FilePath: /algorithm/golang/105_build_tree/build_tree.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

// TreeNode Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func buildTree(preorder []int, inorder []int) *TreeNode {
	if len(preorder) == 0 {
		return nil
	}
	root := &TreeNode{preorder[0], nil, nil}
	stack := []*TreeNode{}
	stack = append(stack, root)
	var inorderIndex int
	for i := 1; i < len(preorder); i++ {
		preorderVal := preorder[i]
		node := stack[len(stack)-1]
		if node.Val != inorder[inorderIndex] {
			node.Left = &TreeNode{preorderVal, nil, nil}
			stack = append(stack, node.Left)
		} else {
			for len(stack) != 0 && stack[len(stack)-1].Val == inorder[inorderIndex] {
				node = stack[len(stack)-1]
				stack = stack[:len(stack)-1]
				inorderIndex++
			}
			node.Right = &TreeNode{preorderVal, nil, nil}
			stack = append(stack, node.Right)
		}
	}
	return root
}

func isSameTree(p *TreeNode, q *TreeNode) bool {
	if p == nil && q == nil {
		return true
	}
	if p == nil || q == nil {
		return false
	}
	if p.Val != q.Val {
		return false
	}
	return isSameTree(p.Left, q.Left) && isSameTree(p.Right, q.Right)
}

func main() {
	tests := []struct {
		preorder []int
		inorder  []int
		ans      *TreeNode
	}{
		{
			[]int{3, 9, 20, 15, 7},
			[]int{9, 3, 15, 20, 7},
			&TreeNode{3, &TreeNode{9, nil, nil}, &TreeNode{20, &TreeNode{15, nil, nil}, &TreeNode{7, nil, nil}}},
		},
		{[]int{-1}, []int{-1}, &TreeNode{-1, nil, nil}},
	}

	for index, test := range tests {
		assert.True(&testing.T{}, isSameTree(buildTree(test.preorder, test.inorder), test.ans), index)
	}
}
