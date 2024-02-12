/*
 * @Date: 2024-02-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-12
 * @FilePath: /algorithm/golang/145_postorder_traversal/postorder_traversal.go
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

func postorderTraversal(root *TreeNode) (res []int) {
	var postorder func(*TreeNode)
	postorder = func(node *TreeNode) {
		if node == nil {
			return
		}
		postorder(node.Left)
		postorder(node.Right)
		res = append(res, node.Val)
	}
	postorder(root)
	return
}

func main() {
	tests := []struct {
		root *TreeNode
		ans  []int
	}{
		{&TreeNode{1, nil, &TreeNode{2, &TreeNode{3, nil, nil}, nil}}, []int{3, 2, 1}},
		{nil, nil},
		{&TreeNode{1, nil, nil}, []int{1}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, postorderTraversal(test.root), index)
	}
}
