/*
 * @Date: 2023-08-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-20
 * @FilePath: /algorithm/golang/2236_check_tree/check_tree.go
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

func checkTree(root *TreeNode) bool {
	return root.Val == root.Left.Val+root.Right.Val
}

func main() {
	tests := []struct {
		root *TreeNode
		ans  bool
	}{
		{&TreeNode{10, &TreeNode{4, nil, nil}, &TreeNode{6, nil, nil}}, true},
		{&TreeNode{5, &TreeNode{3, nil, nil}, &TreeNode{1, nil, nil}}, false},
	}

	for _, item := range tests {
		assert.Equal(&testing.T{}, checkTree(item.root), item.ans, "Not Passed")
	}
}
