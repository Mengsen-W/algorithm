/*
 * @Date: 2024-02-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-21
 * @FilePath: /algorithm/golang/106_build_tree/build_tree.go
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

func buildTree(inorder []int, postorder []int) *TreeNode {
	if len(postorder) == 0 {
		return nil
	}
	root := &TreeNode{Val: postorder[len(postorder)-1]}
	stack := []*TreeNode{root}
	inorderIndex := len(inorder) - 1
	for i := len(postorder) - 2; i >= 0; i-- {
		postorderVal := postorder[i]
		node := stack[len(stack)-1]
		if node.Val != inorder[inorderIndex] {
			node.Right = &TreeNode{Val: postorderVal}
			stack = append(stack, node.Right)
		} else {
			for len(stack) > 0 && stack[len(stack)-1].Val == inorder[inorderIndex] {
				node = stack[len(stack)-1]
				stack = stack[:len(stack)-1]
				inorderIndex--
			}
			node.Left = &TreeNode{Val: postorderVal}
			stack = append(stack, node.Left)
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
			[]int{9, 3, 15, 20, 7},
			[]int{9, 15, 7, 20, 3},
			&TreeNode{3, &TreeNode{9, nil, nil}, &TreeNode{20, &TreeNode{15, nil, nil}, &TreeNode{7, nil, nil}}},
		},
		{[]int{-1}, []int{-1}, &TreeNode{-1, nil, nil}},
	}

	for index, test := range tests {
		assert.True(&testing.T{}, isSameTree(buildTree(test.preorder, test.inorder), test.ans), index)
	}
}
