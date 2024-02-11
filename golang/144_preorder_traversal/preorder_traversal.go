/*
 * @Date: 2024-02-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-11
 * @FilePath: /algorithm/golang/144_preorder_traversal/preorder_traversal.go
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
		root *TreeNode
		ans  []int
	}{
		{&TreeNode{1, nil, &TreeNode{2, &TreeNode{3, nil, nil}, nil}}, []int{1, 2, 3}},
		{nil, nil},
		{&TreeNode{1, nil, &TreeNode{2, nil, nil}}, []int{1, 2}},
		{&TreeNode{1, &TreeNode{2, nil, nil}, nil}, []int{1, 2}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, preorderTraversal(test.root), index)
	}
}
