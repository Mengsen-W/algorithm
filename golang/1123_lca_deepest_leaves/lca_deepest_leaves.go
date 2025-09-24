/*
 * @Date: 2023-09-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-06
 * @FilePath: /algorithm/golang/1123_lca_deepest_leaves/lca_deepest_leaves.go
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

func lcaDeepestLeaves(root *TreeNode) *TreeNode {
	var f func(root *TreeNode) (int, *TreeNode)
	f = func(root *TreeNode) (int, *TreeNode) {
		if root == nil {
			return 0, nil
		}

		d1, lca1 := f(root.Left)
		h2, lca2 := f(root.Right)

		if d1 > h2 {
			return d1 + 1, lca1
		}
		if d1 < h2 {
			return h2 + 1, lca2
		}
		return d1 + 1, root
	}
	_, lca := f(root)
	return lca
}

func isEqual(a, b *TreeNode) bool {
	if a == nil && b == nil {
		return true
	}
	return a.Val == b.Val && isEqual(a.Left, b.Left) && isEqual(a.Right, b.Right)
}

func main() {
	tests := []struct {
		root *TreeNode
		ans  *TreeNode
	}{
		{
			&TreeNode{3, &TreeNode{5, &TreeNode{6, nil, nil}, &TreeNode{2, &TreeNode{7, nil, nil}, &TreeNode{4, nil, nil}}}, &TreeNode{1, &TreeNode{0, nil, nil}, &TreeNode{8, nil, nil}}},
			&TreeNode{2, &TreeNode{7, nil, nil}, &TreeNode{4, nil, nil}},
		},
		{
			&TreeNode{1, nil, nil},
			&TreeNode{1, nil, nil},
		},
		{
			&TreeNode{0, &TreeNode{1, nil, &TreeNode{2, nil, nil}}, &TreeNode{3, nil, nil}},
			&TreeNode{2, nil, nil},
		},
	}

	for index, item := range tests {
		assert.True(&testing.T{}, isEqual(lcaDeepestLeaves(item.root), item.ans), index)
	}
}
