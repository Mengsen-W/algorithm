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

func isEqual(a, b *TreeNode) bool {
	if a == nil && b == nil {
		return true
	}
	return a.Val == b.Val && isEqual(a.Left, b.Left) && isEqual(a.Right, b.Right)
}

func subtreeWithAllDeepest(root *TreeNode) *TreeNode {
	_, lca := f(root)
	return lca
}

func f(root *TreeNode) (int, *TreeNode) {
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

func main() {
	tests := []struct {
		root *TreeNode
		ans  *TreeNode
	}{
		{
			&TreeNode{
				3,
				&TreeNode{5, &TreeNode{6, nil, nil}, &TreeNode{2, &TreeNode{7, nil, nil}, &TreeNode{4, nil, nil}}},
				&TreeNode{1, &TreeNode{0, nil, nil}, &TreeNode{8, nil, nil}},
			},

			&TreeNode{2, &TreeNode{7, nil, nil}, &TreeNode{4, nil, nil}},
		},
		{
			&TreeNode{1, nil, nil},

			&TreeNode{1, nil, nil},
		},
		{
			&TreeNode{
				0,
				&TreeNode{1, nil, &TreeNode{2, nil, nil}},
				&TreeNode{3, nil, nil},
			},

			&TreeNode{2, nil, nil},
		},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, isEqual(subtreeWithAllDeepest(test.root), test.ans), true, index)
	}
}
