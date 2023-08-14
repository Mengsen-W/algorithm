/*
 * @Date: 2023-08-14
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-14
 * @FilePath: /algorithm/golang/617_merge_trees/merge_trees.go
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

func mergeTrees(t1, t2 *TreeNode) *TreeNode {
	if t1 == nil {
		return t2
	}
	if t2 == nil {
		return t1
	}
	t1.Val += t2.Val
	t1.Left = mergeTrees(t1.Left, t2.Left)
	t1.Right = mergeTrees(t1.Right, t2.Right)
	return t1
}

func preorderTraversal(root *TreeNode) (vals []int) {
	var preorder func(*TreeNode)
	preorder = func(node *TreeNode) {
		if node == nil {
			return
		}
		vals = append(vals, node.Val)
		preorder(node.Left)
		preorder(node.Right)
	}
	preorder(root)
	return
}

func main() {
	tests := []struct {
		t1  *TreeNode
		t2  *TreeNode
		ans *TreeNode
	}{
		{
			&TreeNode{1, &TreeNode{3, &TreeNode{5, nil, nil}, nil}, &TreeNode{2, nil, nil}},
			&TreeNode{2, &TreeNode{1, nil, &TreeNode{4, nil, nil}}, &TreeNode{3, nil, &TreeNode{7, nil, nil}}},
			&TreeNode{3, &TreeNode{4, &TreeNode{5, nil, nil}, &TreeNode{4, nil, nil}}, &TreeNode{5, nil, &TreeNode{7, nil, nil}}},
		},
		{
			&TreeNode{1, nil, nil},
			&TreeNode{1, &TreeNode{2, nil, nil}, nil},
			&TreeNode{2, &TreeNode{2, nil, nil}, nil},
		},
	}

	for _, item := range tests {
		assert.Equal(&testing.T{}, preorderTraversal(mergeTrees(item.t1, item.t2)), preorderTraversal(item.ans))
	}
}
