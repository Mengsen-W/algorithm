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

func isSubtree(root *TreeNode, subRoot *TreeNode) bool {
	var same func(p, q *TreeNode) bool
	same = func(p, q *TreeNode) bool {
		if p == nil || q == nil {
			return p == q
		}
		return p.Val == q.Val && same(p.Left, q.Left) && same(p.Right, q.Right)
	}
	if root == nil {
		return false
	}
	return same(root, subRoot) || isSubtree(root.Left, subRoot) || isSubtree(root.Right, subRoot)
}

func main() {
	tests := []struct {
		s   *TreeNode
		t   *TreeNode
		ans bool
	}{
		{
			&TreeNode{3, &TreeNode{4, &TreeNode{1, nil, nil}, &TreeNode{2, nil, nil}}, &TreeNode{5, nil, nil}},
			&TreeNode{4, &TreeNode{1, nil, nil}, &TreeNode{2, nil, nil}},
			true,
		},
		{
			&TreeNode{3, &TreeNode{4, &TreeNode{1, nil, nil}, &TreeNode{2, &TreeNode{0, nil, nil}, nil}}, &TreeNode{5, nil, nil}},
			&TreeNode{4, &TreeNode{1, nil, nil}, &TreeNode{2, nil, nil}},
			false,
		},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, isSubtree(test.s, test.t), index)
	}
}
